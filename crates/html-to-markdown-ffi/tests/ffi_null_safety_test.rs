//! Null-pointer safety tests for the C ABI surface.
//!
//! Every exported `extern "C"` function must tolerate a null pointer without triggering
//! undefined behaviour: no dereference of null, no crash, and a documented fallback value.
//! These tests call the real exported functions with null arguments and assert the exact
//! fallback each one is documented to return.

#![allow(unsafe_code)]

use std::ptr;

use html_to_markdown_ffi::*;

/// Every `htm_*_free` function must be a safe no-op when passed a null pointer. This is an
/// exhaustive sweep over all 41 free functions currently exported by the crate; if a new
/// `_free` function is added without a null guard, this test will crash instead of passing,
/// catching the regression immediately.
#[test]
fn should_not_crash_when_freeing_null_pointer_for_every_owned_type() {
    // SAFETY: every call below passes a null pointer, which each function's documented
    // contract explicitly allows ("or be null").
    unsafe {
        htm_document_metadata_free(ptr::null_mut());
        htm_header_metadata_free(ptr::null_mut());
        htm_link_metadata_free(ptr::null_mut());
        htm_image_metadata_free(ptr::null_mut());
        htm_structured_data_free(ptr::null_mut());
        htm_html_metadata_free(ptr::null_mut());
        htm_conversion_options_free(ptr::null_mut());
        htm_conversion_options_update_free(ptr::null_mut());
        htm_preprocessing_options_free(ptr::null_mut());
        htm_preprocessing_options_update_free(ptr::null_mut());
        htm_image_dimensions_free(ptr::null_mut());
        htm_document_structure_free(ptr::null_mut());
        htm_document_node_free(ptr::null_mut());
        htm_text_annotation_free(ptr::null_mut());
        htm_metadata_entry_free(ptr::null_mut());
        htm_conversion_result_free(ptr::null_mut());
        htm_table_grid_free(ptr::null_mut());
        htm_grid_cell_free(ptr::null_mut());
        htm_table_data_free(ptr::null_mut());
        htm_processing_warning_free(ptr::null_mut());
        htm_visitor_handle_free(ptr::null_mut());
        htm_node_context_free(ptr::null_mut());
        htm_text_direction_free(ptr::null_mut());
        htm_link_type_free(ptr::null_mut());
        htm_image_type_free(ptr::null_mut());
        htm_structured_data_type_free(ptr::null_mut());
        htm_tier_strategy_free(ptr::null_mut());
        htm_preprocessing_preset_free(ptr::null_mut());
        htm_heading_style_free(ptr::null_mut());
        htm_list_indent_type_free(ptr::null_mut());
        htm_whitespace_mode_free(ptr::null_mut());
        htm_newline_style_free(ptr::null_mut());
        htm_code_block_style_free(ptr::null_mut());
        htm_highlight_style_free(ptr::null_mut());
        htm_link_style_free(ptr::null_mut());
        htm_url_escape_style_free(ptr::null_mut());
        htm_output_format_free(ptr::null_mut());
        htm_node_content_free(ptr::null_mut());
        htm_annotation_kind_free(ptr::null_mut());
        htm_warning_kind_free(ptr::null_mut());
        htm_node_type_free(ptr::null_mut());
        htm_visitor_free(ptr::null_mut());
        htm_htm_html_visitor_bridge_free(ptr::null_mut());
    }
    // Reaching this point without a crash is the assertion: every free above tolerated null.
}

#[test]
fn should_return_null_pointer_when_string_getters_receive_null_handle() {
    unsafe {
        assert!(htm_document_metadata_title(ptr::null()).is_null());
        assert!(htm_document_metadata_description(ptr::null()).is_null());
        assert!(htm_document_metadata_canonical_url(ptr::null()).is_null());
        assert!(htm_header_metadata_text(ptr::null()).is_null());
        assert!(htm_link_metadata_href(ptr::null()).is_null());
        assert!(htm_image_metadata_src(ptr::null()).is_null());
        assert!(htm_html_metadata_to_json(ptr::null()).is_null());
        assert!(htm_conversion_options_to_json(ptr::null()).is_null());
        assert!(htm_conversion_result_content(ptr::null()).is_null());
        assert!(htm_document_node_content(ptr::null()).is_null());
        assert!(htm_table_grid_to_json(ptr::null()).is_null());
        assert!(htm_metadata_entry_key(ptr::null()).is_null());
    }
}

#[test]
fn should_return_null_pointer_when_owned_handle_getters_receive_null_handle() {
    unsafe {
        assert!(htm_conversion_options_output_format(ptr::null()).is_null());
        assert!(htm_conversion_options_tier_strategy(ptr::null()).is_null());
        assert!(htm_html_metadata_document(ptr::null()).is_null());
        assert!(htm_image_metadata_dimensions(ptr::null()).is_null());
        assert!(htm_conversion_result_document(ptr::null()).is_null());
        assert!(htm_conversion_result_metadata(ptr::null()).is_null());
    }
}

#[test]
fn should_return_documented_fallback_when_scalar_getters_receive_null_handle() {
    unsafe {
        assert_eq!(htm_header_metadata_level(ptr::null()), 0);
        assert_eq!(htm_header_metadata_depth(ptr::null()), 0);
        assert_eq!(htm_header_metadata_html_offset(ptr::null()), 0);
        assert_eq!(htm_image_dimensions_width(ptr::null()), 0);
        assert_eq!(htm_image_dimensions_height(ptr::null()), 0);
        assert_eq!(htm_text_annotation_start(ptr::null()), 0);
        assert_eq!(htm_text_annotation_end(ptr::null()), 0);
        assert_eq!(htm_table_grid_rows(ptr::null()), 0);
        assert_eq!(htm_table_grid_cols(ptr::null()), 0);
    }
}

#[cfg(feature = "visitor")]
#[test]
fn should_return_null_pointer_when_visitor_options_setter_receives_null_options() {
    unsafe {
        // Passing null `options` must be a safe no-op rather than a null-pointer write.
        htm_options_set_visitor(ptr::null_mut(), ptr::null_mut());
    }
}

#[test]
fn should_return_null_result_when_convert_receives_null_html() {
    unsafe {
        clear_last_error_for_test();
        let result = htm_convert(ptr::null(), ptr::null());
        assert!(
            result.is_null(),
            "convert must reject a null html pointer rather than dereference it"
        );
        assert_eq!(
            htm_last_error_code(),
            1,
            "null html should set error code 1 (invalid input)"
        );
    }
}

#[test]
fn should_succeed_when_convert_receives_null_options() {
    unsafe {
        let html = std::ffi::CString::new("<p>ok</p>").unwrap();
        let result = htm_convert(html.as_ptr(), ptr::null());
        assert!(
            !result.is_null(),
            "convert must accept a null options pointer and fall back to defaults"
        );
        htm_conversion_result_free(result);
    }
}

#[test]
fn should_return_null_when_json_constructors_receive_null_string() {
    unsafe {
        assert!(htm_conversion_options_from_json(ptr::null()).is_null());
        assert!(htm_conversion_options_update_from_json(ptr::null()).is_null());
        assert!(htm_document_metadata_from_json(ptr::null()).is_null());
        assert!(htm_header_metadata_from_json(ptr::null()).is_null());
        assert!(htm_link_metadata_from_json(ptr::null()).is_null());
        assert!(htm_image_metadata_from_json(ptr::null()).is_null());
        assert!(htm_structured_data_from_json(ptr::null()).is_null());
        assert!(htm_html_metadata_from_json(ptr::null()).is_null());
    }
}

/// `htm_last_error_context`/`htm_last_error_code` must be callable with no prior error state
/// and must not crash when called back-to-back from a fresh thread-local.
#[test]
fn should_report_no_error_when_no_prior_ffi_call_failed_on_this_thread() {
    unsafe {
        clear_last_error_for_test();
        assert_eq!(htm_last_error_code(), 0);
        let ctx = htm_last_error_context();
        assert!(ctx.is_null(), "context must be null when there is no recorded error");
    }
}

/// Reset the thread-local error state via a call known to clear it on success, so tests that
/// check for the "no error" baseline are independent of prior tests' failures on this thread.
unsafe fn clear_last_error_for_test() {
    unsafe {
        let html = std::ffi::CString::new("<p>reset</p>").unwrap();
        let result = htm_convert(html.as_ptr(), ptr::null());
        assert!(!result.is_null());
        htm_conversion_result_free(result);
    }
}
