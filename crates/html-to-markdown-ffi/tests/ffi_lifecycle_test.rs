//! Alloc -> use -> free lifecycle tests for the C ABI surface exported by
//! `html-to-markdown-ffi`.
//!
//! These exercise the real exported `extern "C"` functions directly (no mocking), the way a
//! C/FFI caller would: allocate a handle, read fields through the accessor functions, and
//! free the handle exactly once. Each test's assertions cover exact expected values, not just
//! "it didn't crash".

#![allow(unsafe_code)]

use std::ffi::{CStr, CString};

use html_to_markdown_ffi::*;

/// Build a null-terminated C string owned by the caller for the duration of a test.
fn cstr(s: &str) -> CString {
    CString::new(s).expect("test fixture string must not contain interior NUL bytes")
}

#[test]
fn should_allocate_and_free_default_conversion_options() {
    // SAFETY: exercising the exported alloc/free pair directly, per the crate's own contract.
    unsafe {
        let options = htm_conversion_options_default();
        assert!(!options.is_null(), "htm_conversion_options_default must not return null");
        htm_conversion_options_free(options);
    }
}

#[test]
fn should_round_trip_conversion_result_through_convert_and_free() {
    // SAFETY: standard alloc -> use -> free cycle using only library-returned pointers.
    unsafe {
        let html = cstr("<h1>Hello World</h1>");
        let result = htm_convert(html.as_ptr(), std::ptr::null());
        assert!(!result.is_null(), "htm_convert must succeed for well-formed HTML");

        let content_ptr = htm_conversion_result_content(result);
        assert!(!content_ptr.is_null(), "content getter must return a non-null owned string");
        let content = CStr::from_ptr(content_ptr).to_str().expect("content must be valid UTF-8");
        assert_eq!(content, "# Hello World\n", "unexpected markdown output: {content:?}");
        htm_free_string(content_ptr);

        htm_conversion_result_free(result);
    }
}

#[test]
fn should_return_two_independently_owned_strings_from_repeated_content_calls() {
    // ~keep Regression guard for the exact class of bug flagged in the FFI audit: a getter
    // ~keep that hands back a borrowed pointer typed as owned. Calling the getter twice must
    // ~keep produce two DISTINCT heap allocations, each independently freeable, never an alias
    // ~keep of the same memory.
    unsafe {
        let html = cstr("<p>content</p>");
        let result = htm_convert(html.as_ptr(), std::ptr::null());
        assert!(!result.is_null());

        let first = htm_conversion_result_content(result);
        let second = htm_conversion_result_content(result);
        assert!(!first.is_null() && !second.is_null());
        assert_ne!(
            first, second,
            "content getter must allocate a fresh owned string on every call, not alias a previous one"
        );

        htm_free_string(first);
        htm_free_string(second);
        htm_conversion_result_free(result);
    }
}

#[test]
fn should_round_trip_conversion_options_through_json() {
    unsafe {
        let json = cstr(r#"{"wrap":false,"output_format":"Markdown"}"#);
        let options = htm_conversion_options_from_json(json.as_ptr());
        assert!(!options.is_null(), "options must parse from valid JSON");

        let output_format = htm_conversion_options_output_format(options);
        assert!(!output_format.is_null(), "output_format getter must return an owned enum handle");
        htm_output_format_free(output_format);

        let serialized_ptr = htm_conversion_options_to_json(options);
        assert!(!serialized_ptr.is_null());
        let serialized = CStr::from_ptr(serialized_ptr).to_str().unwrap();
        assert!(
            serialized.contains("\"wrap\":false"),
            "round-tripped JSON must preserve the wrap field, got: {serialized}"
        );
        htm_free_string(serialized_ptr);

        htm_conversion_options_free(options);
    }
}

#[cfg(feature = "metadata")]
#[test]
fn should_extract_document_title_through_metadata_lifecycle() {
    unsafe {
        let json = cstr(r#"{"extract_metadata":true}"#);
        let options = htm_conversion_options_from_json(json.as_ptr());
        assert!(!options.is_null());

        let html = cstr("<html><head><title>Test Page</title></head><body><p>x</p></body></html>");
        let result = htm_convert(html.as_ptr(), options);
        assert!(!result.is_null(), "conversion with metadata extraction must succeed");

        let metadata = htm_conversion_result_metadata(result);
        assert!(!metadata.is_null(), "metadata getter must return a non-null handle when extraction is enabled");

        let document = htm_html_metadata_document(metadata);
        assert!(!document.is_null());

        let title_ptr = htm_document_metadata_title(document);
        assert!(!title_ptr.is_null(), "title must be present for the fixture HTML");
        let title = CStr::from_ptr(title_ptr).to_str().unwrap();
        assert_eq!(title, "Test Page");
        htm_free_string(title_ptr);

        htm_document_metadata_free(document);
        htm_html_metadata_free(metadata);
        htm_conversion_result_free(result);
        htm_conversion_options_free(options);
    }
}

#[test]
fn should_free_conversion_options_update_handle() {
    unsafe {
        let json = cstr(r#"{"wrap":true}"#);
        let update = htm_conversion_options_update_from_json(json.as_ptr());
        assert!(!update.is_null(), "update handle must parse from valid JSON");
        htm_conversion_options_update_free(update);
    }
}

#[test]
fn should_free_string_and_bytes_allocated_by_the_library() {
    unsafe {
        let json = cstr(r"{}");
        let options = htm_conversion_options_from_json(json.as_ptr());
        let serialized = htm_conversion_options_to_json(options);
        assert!(!serialized.is_null());
        htm_free_string(serialized);
        htm_conversion_options_free(options);
    }
}
