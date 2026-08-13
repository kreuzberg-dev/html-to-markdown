// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::{ConversionOptions, NewlineStyle};

/// Regression test for issue #454: a `<div>` continuation inside a table cell must render
/// identically to a `<br>` (issue #453's settled rule) regardless of `newline_style` and
/// regardless of whether the source HTML has a literal newline between the two `<div>`s.
/// Covers the full `br_in_tables` x `newline_style` x with/without-source-LF matrix.
#[test]
fn should_ignore_newline_style_and_source_whitespace_for_div_continuation_in_table_cell() {
    let without_source_lf = "<table><tr><td><div>A</div><div>B</div></td></tr></table>";
    let with_source_lf = "<table><tr><td><div>A</div>\n<div>B</div></td></tr></table>";

    for (html, has_source_lf) in [(without_source_lf, false), (with_source_lf, true)] {
        for newline_style in [NewlineStyle::Spaces, NewlineStyle::Backslash] {
            let options = ConversionOptions {
                br_in_tables: false,
                newline_style,
                compact_tables: true,
                ..Default::default()
            };
            let result = convert(html, Some(options)).unwrap();
            assert_eq!(
                result, "| A B |\n| --- |\n",
                "br_in_tables=false, newline_style={newline_style:?}, source_lf={has_source_lf}: {result}"
            );

            let options = ConversionOptions {
                br_in_tables: true,
                newline_style,
                compact_tables: true,
                ..Default::default()
            };
            let result = convert(html, Some(options)).unwrap();
            assert_eq!(
                result, "| A<br>B |\n| --- |\n",
                "br_in_tables=true, newline_style={newline_style:?}, source_lf={has_source_lf}: {result}"
            );
        }
    }
}

/// Regression test for issue #454: a `<p>` continuation inside a table cell must render
/// identically to a `<br>` (issue #453's settled rule) regardless of `newline_style` and
/// regardless of whether the source HTML has a literal newline between the two `<p>`s.
/// Before the fix, `<p>` always emitted `<br>` and ignored `br_in_tables` entirely.
/// Covers the full `br_in_tables` x `newline_style` x with/without-source-LF matrix.
#[test]
fn should_ignore_newline_style_and_source_whitespace_for_paragraph_continuation_in_table_cell() {
    let without_source_lf = "<table><tr><td><p>A</p><p>B</p></td></tr></table>";
    let with_source_lf = "<table><tr><td><p>A</p>\n<p>B</p></td></tr></table>";

    for (html, has_source_lf) in [(without_source_lf, false), (with_source_lf, true)] {
        for newline_style in [NewlineStyle::Spaces, NewlineStyle::Backslash] {
            let options = ConversionOptions {
                br_in_tables: false,
                newline_style,
                compact_tables: true,
                ..Default::default()
            };
            let result = convert(html, Some(options)).unwrap();
            assert_eq!(
                result, "| A B |\n| --- |\n",
                "br_in_tables=false, newline_style={newline_style:?}, source_lf={has_source_lf}: {result}"
            );

            let options = ConversionOptions {
                br_in_tables: true,
                newline_style,
                compact_tables: true,
                ..Default::default()
            };
            let result = convert(html, Some(options)).unwrap();
            assert_eq!(
                result, "| A<br>B |\n| --- |\n",
                "br_in_tables=true, newline_style={newline_style:?}, source_lf={has_source_lf}: {result}"
            );
        }
    }
}

/// Regression test for issue #454: with `br_in_tables=false` and `newline_style=Backslash`,
/// a `<div>` continuation used to fall through to `newline_style`, leaking a literal `\`
/// into the cell text. A table cell must collapse the continuation to a single space instead.
#[test]
fn should_not_leak_literal_backslash_from_div_continuation_with_backslash_newline_style() {
    let html = "<table><tr><td><div>A</div><div>B</div></td></tr></table>";
    let options = ConversionOptions {
        br_in_tables: false,
        newline_style: NewlineStyle::Backslash,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| A B |\n| --- |\n");
    assert!(
        !result.contains('\\'),
        "no literal backslash should leak into the cell: {result}"
    );
}

/// Regression test for issue #454: with `br_in_tables=false` and `newline_style=Spaces`, a
/// `<div>` continuation used to emit a raw `"  \n"` hard break into the cell, which the
/// cell post-pass could not fully clean up, leaving a stray extra space. A table cell must
/// collapse the continuation to exactly one space instead.
#[test]
fn should_not_leave_stray_space_from_div_continuation_with_spaces_newline_style() {
    let html = "<table><tr><td><div>A</div><div>B</div></td></tr></table>";
    let options = ConversionOptions {
        br_in_tables: false,
        newline_style: NewlineStyle::Spaces,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| A B |\n| --- |\n");
}

/// Regression test for issue #454: `<p>` continuations previously ignored `br_in_tables`
/// entirely and always emitted a literal `<br>`. With the option disabled, the continuation
/// must collapse to a single space instead.
#[test]
fn should_honor_br_in_tables_false_for_paragraph_continuation() {
    let html = "<table><tr><td><p>A</p><p>B</p></td></tr></table>";
    let options = ConversionOptions {
        br_in_tables: false,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| A B |\n| --- |\n");
    assert!(
        !result.contains("<br>"),
        "br_in_tables=false must not emit <br>: {result}"
    );
}

/// Regression test for issue #454: a `<p>` (or `<div>`) that is the FIRST child of a table
/// cell must not gain a leading space or a leading `<br>` — the continuation logic only
/// applies between siblings, not before the cell's first content.
#[test]
fn should_not_add_leading_separator_for_first_child_block_in_table_cell() {
    for html in [
        "<table><tr><td><p>A</p></td></tr></table>",
        "<table><tr><td><div>A</div></td></tr></table>",
    ] {
        for br_in_tables in [false, true] {
            let options = ConversionOptions {
                br_in_tables,
                compact_tables: true,
                ..Default::default()
            };
            let result = convert(html, Some(options)).unwrap();
            assert_eq!(
                result, "| A |\n| --- |\n",
                "html={html}, br_in_tables={br_in_tables}: {result}"
            );
        }
    }
}

/// Regression test for issue #454: `<div>`/`<p>` continuations must interoperate with a
/// literal `<br>` inside the same cell, producing one separator per boundary rather than
/// double separators or dropped content.
#[test]
fn should_interoperate_with_br_when_mixed_with_block_continuations_in_table_cell() {
    let html = "<table><tr><td>A<br/>B<div>C</div></td></tr></table>";

    let options = ConversionOptions {
        br_in_tables: false,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| A B C |\n| --- |\n");

    let options = ConversionOptions {
        br_in_tables: true,
        compact_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();
    assert_eq!(result, "| A<br>B<br>C |\n| --- |\n");
}

/// Regression test for issue #454: a `<p>` nested inside a `<div>` (and vice versa) inside a
/// table cell must still resolve to a single separator between "A" and "B" rather than a raw
/// newline or a doubled break, and must never split the row across physical lines.
#[test]
fn should_handle_nested_div_and_paragraph_continuations_in_table_cell() {
    for html in [
        "<table><tr><td><div>A<p>B</p></div></td></tr></table>",
        "<table><tr><td><p>A<div>B</div></p></td></tr></table>",
    ] {
        let options = ConversionOptions {
            br_in_tables: false,
            compact_tables: true,
            ..Default::default()
        };
        let result = convert(html, Some(options)).unwrap();
        assert_eq!(result, "| A B |\n| --- |\n", "html={html}: {result}");
        assert_eq!(
            result.lines().count(),
            2,
            "row must stay on one physical line: {result}"
        );

        let options = ConversionOptions {
            br_in_tables: true,
            compact_tables: true,
            ..Default::default()
        };
        let result = convert(html, Some(options)).unwrap();
        assert_eq!(result, "| A<br>B |\n| --- |\n", "html={html}: {result}");
        assert_eq!(
            result.lines().count(),
            2,
            "row must stay on one physical line: {result}"
        );
    }
}

/// Regression test for issue #454: a source newline preceding a `<div>` or `<p>` continuation
/// must never survive into the rendered cell as a real `\n`, which would split the row's pipe
/// syntax across physical lines and corrupt the table structure.
#[test]
fn should_not_leak_real_newline_from_source_whitespace_around_block_continuations() {
    for html in [
        "<table><tr><td><div>A</div>\n<div>B</div></td></tr></table>",
        "<table><tr><td><p>A</p>\n<p>B</p></td></tr></table>",
    ] {
        let options = ConversionOptions {
            br_in_tables: true,
            compact_tables: true,
            ..Default::default()
        };
        let result = convert(html, Some(options)).unwrap();
        assert_eq!(result, "| A<br>B |\n| --- |\n", "html={html}: {result}");
        assert_eq!(
            result.lines().count(),
            2,
            "a source newline must not split the table row across physical lines: {result}"
        );
    }
}
