// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

//! Tests for the `max_depth` recursion-safety option.

use html_to_markdown_rs::{ConversionOptions, WarningKind};

fn convert_with_options(html: &str, options: ConversionOptions) -> String {
    html_to_markdown_rs::convert(html, Some(options))
        .expect("conversion should not fail")
        .content
        .unwrap_or_default()
}

fn nested_divs(depth: usize, inner: &str) -> String {
    let mut html = inner.to_string();
    for _ in 0..depth {
        html = format!("<div>{html}</div>");
    }
    html
}

/// With the default `max_depth: None`, ordinary nesting below the native stack
/// safety limit should be fully converted.
#[test]
fn test_max_depth_none_converts_reasonably_nested_content() {
    let mut html = String::from("<p>deep</p>");
    for _ in 0..32 {
        html = format!("<div>{html}</div>");
    }

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: None,
        ..Default::default()
    };

    let result = convert_with_options(&html, options);
    assert!(
        result.contains("deep"),
        "Deeply nested text should be present when max_depth is None. Got:\n{result}"
    );
}

/// With `max_depth: Some(2)`, block elements at depth 2 are not visited, so
/// their text content is excluded from the output.
#[test]
fn test_max_depth_truncates_at_limit() {
    let html = "<div><p>shallow</p><div><p>deep</p></div></div>";

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: Some(3),
        ..Default::default()
    };

    let result = convert_with_options(html, options);
    assert!(
        result.contains("shallow"),
        "Content at depth < max_depth should be present. Got:\n{result}"
    );
    assert!(
        !result.contains("deep"),
        "Content at depth >= max_depth should be absent. Got:\n{result}"
    );
}

/// Issue #434: callers can raise the ceiling above the native default (64) by
/// setting an explicit `max_depth`, recovering content in deeply-nested email HTML.
#[test]
fn test_max_depth_can_be_raised_above_native_default_issue_434() {
    let html = nested_divs(90, "<p>deep-content</p>");

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: Some(256),
        ..Default::default()
    };

    let result = html_to_markdown_rs::convert(&html, Some(options)).expect("conversion should not fail");
    assert!(
        result.content.unwrap_or_default().contains("deep-content"),
        "Raising max_depth above 64 must recover content nested deeper than the native default."
    );
    // ~keep Nothing was truncated (90 < 256), so no depth warning should be emitted.
    assert!(
        !result
            .warnings
            .iter()
            .any(|w| w.kind == WarningKind::DepthLimitExceeded),
        "No DepthLimitExceeded warning expected when nothing is truncated: {:?}",
        result.warnings
    );
}

/// Issue #434: when the depth limit truncates content, a `DepthLimitExceeded`
/// warning is surfaced (previously the subtree was dropped silently).
#[test]
fn test_depth_truncation_emits_warning_issue_434() {
    let html = nested_divs(90, "<p>deep-content</p>");

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: None, // ~keep native default 64 truncates at depth 64
        ..Default::default()
    };

    let result = html_to_markdown_rs::convert(&html, Some(options)).expect("conversion should not fail");
    assert!(
        !result.content.clone().unwrap_or_default().contains("deep-content"),
        "Content nested beyond the native default should be truncated."
    );
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.kind == WarningKind::DepthLimitExceeded),
        "Expected a DepthLimitExceeded warning when content is dropped: {:?}",
        result.warnings
    );
}

/// With `max_depth: Some(0)`, no nodes are processed and the output is empty or whitespace only.
#[test]
fn test_max_depth_zero_produces_empty() {
    let html = "<p>hello</p>";

    let options = ConversionOptions {
        extract_metadata: false,
        max_depth: Some(0),
        ..Default::default()
    };

    let result = convert_with_options(html, options);
    assert!(
        result.trim().is_empty(),
        "max_depth: Some(0) should produce no output. Got:\n{result}"
    );
}
