//! Verifies that a panic inside a visitor callback is contained at the C ABI boundary.
//!
//! A visitor callback is, from the FFI's point of view, an arbitrary `extern "C"` function
//! pointer supplied by the caller. `HtmVisitorCallbacks` fields are typed as plain
//! `extern "C" fn` (the non-unwind ABI), not `extern "C-unwind"`. Rust's runtime therefore
//! refuses to let a panic unwind out of such a function: it converts the unwind into a hard
//! process abort ("thread caused non-unwinding panic. aborting.") rather than allow undefined
//! behaviour to propagate into the C caller. This is Rust's own memory-safety guarantee, not
//! a bug — but it means a real panic inside a raw C callback crashes the whole process, and
//! that trade-off (memory-safe abort, not silent UB, not a graceful `Err`) needs to be an
//! exercised, verified property, not an assumption. We prove it out-of-process: an in-process
//! call would abort the test harness itself.
//!
//! For well-behaved bindings, this is exactly why the shim generated around a real language
//! callback (Python, Node, etc.) must run the guest callback under its own `catch_unwind`
//! *before* returning across the `extern "C"` boundary — see the core-level regression
//! coverage in `crates/html-to-markdown/tests/visitor_mutex_poison_test.rs`, which proves
//! that a panic caught before it reaches an `extern "C"` frame lets the same visitor handle
//! be reused for later, unrelated conversions.

#![cfg(feature = "visitor")]
#![allow(unsafe_code)]

use std::ffi::{CString, c_char, c_void};

use html_to_markdown_ffi::*;

const CHILD_PROCESS_ENV_VAR: &str = "HTM_FFI_PANIC_BOUNDARY_CHILD";

unsafe extern "C" fn panicking_visit_text(
    _ctx: *const HtmContext,
    _user_data: *mut c_void,
    _text: *const c_char,
    _out_custom: *mut *mut c_char,
    _out_len: *mut usize,
) -> i32 {
    panic!("intentional panic from a visitor callback for test coverage");
}

/// SAFETY: `HtmVisitorCallbacks` is a C-style struct of `Option<fn ptr>` and one raw
/// pointer; the all-zero bit pattern is a valid representation of "every field is
/// `None`/null", so zero-initialising it does not produce an invalid value.
#[allow(clippy::missing_const_for_fn)]
unsafe fn empty_callbacks() -> HtmVisitorCallbacks {
    unsafe { std::mem::zeroed() }
}

/// Drive a real `htm_convert` call through a visitor callback that panics. Only ever invoked
/// in the child process spawned by the test below — calling this directly aborts the process.
unsafe fn trigger_callback_panic_through_ffi() {
    unsafe {
        let mut callbacks = empty_callbacks();
        callbacks.visit_text = Some(panicking_visit_text);
        let visitor = htm_visitor_create(&raw const callbacks);
        let options = htm_conversion_options_default();
        htm_options_set_visitor(options, visitor);
        let html = CString::new("<p>text that triggers visit_text</p>").unwrap();
        let _ = htm_convert(html.as_ptr(), options);
    }
}

#[test]
fn should_abort_process_rather_than_return_across_the_c_boundary_when_callback_panics() {
    if std::env::var_os(CHILD_PROCESS_ENV_VAR).is_some() {
        // SAFETY: this branch only runs in the re-exec'd child process; the parent never
        // reaches past the assertions below.
        unsafe {
            trigger_callback_panic_through_ffi();
        }
        panic!("unreachable: the FFI call above must have aborted the process before this point");
    }

    let exe = std::env::current_exe().expect("test binary must have a resolvable path");
    let output = std::process::Command::new(exe)
        .args([
            "--exact",
            "should_abort_process_rather_than_return_across_the_c_boundary_when_callback_panics",
            "--nocapture",
        ])
        .env(CHILD_PROCESS_ENV_VAR, "1")
        .output()
        .expect("failed to spawn child test process");

    assert!(
        !output.status.success(),
        "child process must not exit successfully: it triggers a callback panic through the C ABI"
    );

    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        assert_eq!(
            output.status.signal(),
            Some(6),
            "expected SIGABRT (6) from Rust's non-unwind-ABI guard, not an ordinary panic exit; \
             status: {:?}, stderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("non-unwinding panic") || stderr.contains("intentional panic from a visitor callback"),
        "expected the abort diagnostic or the original panic message in stderr, got: {stderr}"
    );
}
