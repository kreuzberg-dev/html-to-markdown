// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Regression tests for the visitor `Mutex` poison latch (xberg-io/html-to-markdown#28).
//!
//! `ConversionOptions::visitor` is an `Arc<Mutex<dyn HtmlVisitor + Send>>` that callers are
//! expected to reuse across many independent `convert()` calls (e.g. one visitor instance
//! per converter object in a language binding). If a visitor callback panics while the
//! `Mutex` guard is held, `std::sync::Mutex` poisons itself. Without recovery, every later
//! `convert()` call that reuses the same handle would fail forever, even for completely
//! unrelated HTML that never touches the panicking code path.

#![cfg(feature = "visitor")]

use html_to_markdown_rs::visitor::{HtmlVisitor, NodeContext, VisitResult, VisitorHandle};
use html_to_markdown_rs::{ConversionOptions, convert};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct PanicOnFirstTextVisitor {
    calls: usize,
}

impl HtmlVisitor for PanicOnFirstTextVisitor {
    fn visit_text(&mut self, _ctx: &NodeContext<'_>, text: &str) -> VisitResult {
        self.calls += 1;
        panic!("intentional panic for test: {text}");
    }
}

#[derive(Debug, Default)]
struct CountingVisitor {
    text_calls: usize,
}

impl HtmlVisitor for CountingVisitor {
    fn visit_text(&mut self, _ctx: &NodeContext<'_>, _text: &str) -> VisitResult {
        self.text_calls += 1;
        VisitResult::Continue
    }
}

/// A panicking callback fails only its own conversion: `convert()` returns an `Err` instead
/// of unwinding out of the call.
#[test]
fn should_return_error_when_visitor_callback_panics() {
    let visitor: VisitorHandle = Arc::new(Mutex::new(PanicOnFirstTextVisitor::default()));
    let options = ConversionOptions::builder().visitor(Some(visitor)).build();

    let result = convert("<p>trigger panic</p>", Some(options));

    assert!(
        result.is_err(),
        "expected convert() to catch the panicking callback and return Err, got {result:?}"
    );
}

/// A subsequent, independent conversion that reuses the SAME visitor handle after a
/// panicking callback must still succeed — the poisoned `Mutex` must not latch permanently.
#[test]
fn should_succeed_when_reusing_visitor_handle_after_panic() {
    let visitor: VisitorHandle = Arc::new(Mutex::new(PanicOnFirstTextVisitor::default()));
    let options = ConversionOptions::builder().visitor(Some(Arc::clone(&visitor))).build();

    let first = convert("<p>trigger panic</p>", Some(options));
    assert!(first.is_err(), "first conversion should have failed from the panic");

    // ~keep A fresh, unrelated document reusing the exact same `Arc<Mutex<_>>` handle.
    let second_visitor: VisitorHandle = Arc::new(Mutex::new(CountingVisitor::default()));
    let second_options = ConversionOptions::builder().visitor(Some(second_visitor)).build();
    let second = convert("<p>unrelated document</p>", Some(second_options));
    let second = second.expect("second, unrelated conversion must succeed after the first one panicked");
    assert_eq!(
        second.content.unwrap_or_default(),
        "unrelated document\n",
        "unrelated conversion should have converted normally"
    );

    // ~keep Reuse the exact same handle that caused the panic, on non-panicking input, to
    // ~keep prove the poison flag on THAT `Mutex` was cleared rather than merely working
    // ~keep around it with a different handle.
    let third_options = ConversionOptions::builder().visitor(Some(Arc::clone(&visitor))).build();
    // ~keep No text nodes here, so `PanicOnFirstTextVisitor::visit_text` is never invoked.
    let third = convert("<hr>", Some(third_options));
    assert!(
        third.is_ok(),
        "reusing the same visitor handle that previously panicked must succeed once the panicking node is gone, got {third:?}"
    );
}

/// The visitor `Mutex` itself is not poisoned after `convert()` catches a panic: locking it
/// directly (bypassing `convert()`) must succeed.
#[test]
fn should_clear_mutex_poison_flag_when_callback_panics() {
    let visitor: VisitorHandle = Arc::new(Mutex::new(PanicOnFirstTextVisitor::default()));
    let options = ConversionOptions::builder().visitor(Some(Arc::clone(&visitor))).build();

    let _ = convert("<p>trigger panic</p>", Some(options));

    let poisoned = visitor.lock().is_err();
    assert!(
        !poisoned,
        "visitor Mutex should not be poisoned after convert() caught the panic"
    );
}
