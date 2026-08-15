// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

use html_to_markdown_rs::{ConversionOptions, WhitespaceMode};

fn convert(html: &str, opts: Option<ConversionOptions>) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

fn strict_cell_options(br_in_tables: bool) -> ConversionOptions {
    ConversionOptions {
        br_in_tables,
        whitespace_mode: WhitespaceMode::Strict,
        compact_tables: true,
        ..Default::default()
    }
}

/// Regression test for issue #457: the reported repro. Under `WhitespaceMode::Strict` the
/// plain-text cell path returned the text verbatim, so a source newline reached the rendered
/// row and split it across physical lines. `br_in_tables: true` disabled the whole-cell
/// backstop that was accidentally hiding this everywhere else.
#[test]
fn should_fold_newline_in_plain_cell_text_when_strict_mode_and_br_in_tables_are_both_set() {
    let html = "<table><tr><td>a\nb</td></tr></table>";
    let result = convert(html, Some(strict_cell_options(true))).unwrap();
    assert_eq!(result, "| a b |\n| --- |\n", "actual: {result:?}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result:?}"
    );
}

/// The same fold must apply with `br_in_tables: false`, where the whole-cell backstop also
/// happens to catch it — pinning the behaviour so it does not silently depend on the backstop.
#[test]
fn should_fold_newline_in_plain_cell_text_when_strict_mode_and_br_in_tables_is_false() {
    let html = "<table><tr><td>a\nb</td></tr></table>";
    let result = convert(html, Some(strict_cell_options(false))).unwrap();
    assert_eq!(result, "| a b |\n| --- |\n", "actual: {result:?}");
}

/// A cell whose children are all text nodes never reaches `text_node.rs` — it is rendered by
/// the bare-text branch of `render_cell_text`. A cell containing an inline *element* takes the
/// other path. Both carried the same `Strict` hole, so both are pinned; the issue named only
/// the `text_node.rs` one.
#[test]
fn should_fold_newline_in_cell_text_when_strict_mode_and_cell_contains_an_inline_element() {
    let html = "<table><tr><td><span>a\nb</span></td></tr></table>";
    let result = convert(html, Some(strict_cell_options(true))).unwrap();
    assert_eq!(result, "| a b |\n| --- |\n", "actual: {result:?}");
    assert_eq!(
        result.lines().count(),
        2,
        "table row must stay on one physical line: {result:?}"
    );
}

/// The fold must survive inline formatting that wraps the text in emphasis markers.
#[test]
fn should_fold_newline_in_cell_text_when_strict_mode_and_text_is_emphasised() {
    let html = "<table><tr><td><em>a\nb</em></td></tr></table>";
    let result = convert(html, Some(strict_cell_options(true))).unwrap();
    assert_eq!(result, "| *a b* |\n| --- |\n", "actual: {result:?}");
}

/// The HTML5 preprocessing step normalizes a raw `\r`/`\r\n` in the source to `\n` before the
/// parser sees it, so a literal carriage return can only arrive via a numeric character
/// reference. A decoded `\r` must fold too, and a decoded `\r\n` pair must fold to one space.
#[test]
fn should_fold_decoded_carriage_returns_in_cell_text_under_strict_whitespace_mode() {
    let cr = convert(
        "<table><tr><td>a&#13;b</td></tr></table>",
        Some(strict_cell_options(true)),
    )
    .unwrap();
    assert_eq!(cr, "| a b |\n| --- |\n", "actual: {cr:?}");

    let crlf = convert(
        "<table><tr><td>a&#13;&#10;b</td></tr></table>",
        Some(strict_cell_options(true)),
    )
    .unwrap();
    assert_eq!(crlf, "| a b |\n| --- |\n", "actual: {crlf:?}");
}

/// `Strict` still means strict: only the structurally impossible line break is folded, and
/// every other whitespace byte in the cell is preserved exactly. This is the guard that keeps
/// the #457 fix from degrading into `Normalized`.
#[test]
fn should_preserve_repeated_spaces_in_cell_text_under_strict_whitespace_mode() {
    let html = "<table><tr><td>a   b</td></tr></table>";
    let result = convert(html, Some(strict_cell_options(true))).unwrap();
    assert_eq!(result, "| a   b |\n| --- |\n", "actual: {result:?}");
}

/// Text outside any table cell must keep its newline verbatim under `Strict` — a regression
/// guard against the in-cell fold leaking into the general text path.
#[test]
fn should_preserve_newline_verbatim_outside_table_cell_under_strict_whitespace_mode() {
    let options = ConversionOptions {
        whitespace_mode: WhitespaceMode::Strict,
        ..Default::default()
    };
    let result = convert("<p>a\nb</p>", Some(options)).unwrap();
    assert_eq!(result, "a\nb\n", "actual: {result:?}");
}
