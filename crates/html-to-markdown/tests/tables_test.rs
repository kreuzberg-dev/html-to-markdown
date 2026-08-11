// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use html_to_markdown_rs::ConversionOptions;

#[test]
fn test_basic_table() {
    let html = r"<table>
    <tr><th>Header 1</th><th>Header 2</th></tr>
    <tr><td>Cell 1</td><td>Cell 2</td></tr>
    </table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Header 1"), "header row missing: {result}");
    assert!(result.contains("| Header 2"), "header row missing: {result}");
    assert!(result.contains("| Cell 1"), "cell row missing: {result}");
    assert!(result.contains("| Cell 2"), "cell row missing: {result}");
    assert!(result.contains("| ---"), "separator row missing: {result}");
}

#[test]
fn test_table_with_sections() {
    let html = r"<table>
        <thead>
            <tr><th>Name</th><th>Age</th></tr>
        </thead>
        <tbody>
            <tr><td>John</td><td>25</td></tr>
            <tr><td>Jane</td><td>30</td></tr>
        </tbody>
        <tfoot>
            <tr><td>Total</td><td>2</td></tr>
        </tfoot>
    </table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Name"), "Name column missing: {result}");
    assert!(result.contains("| Age"), "Age column missing: {result}");
    assert!(result.contains("| John"), "John row missing: {result}");
    assert!(result.contains("| 25"), "25 cell missing: {result}");
    assert!(result.contains("| Jane"), "Jane row missing: {result}");
    assert!(result.contains("| 30"), "30 cell missing: {result}");
    assert!(result.contains("| Total"), "Total row missing: {result}");
    assert!(result.contains("| 2"), "2 cell missing: {result}");
}

#[test]
fn test_table_caption() {
    let html = r"<table><caption>Table Caption</caption><tr><td>Data</td></tr></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("*Table Caption*"));
    assert!(result.contains("| Data |"));
}

#[test]
fn test_table_rowspan() {
    let html = r#"<table>
<tr><th>Header 1</th><th>Header 2</th></tr>
<tr><td rowspan="2">Spanning cell</td><td>
    <div>First row content</div>
    <div>Second line</div>
</td></tr>
<tr><td>
    <div>Next row</div>
    <div>More content</div>
</td></tr>
</table>"#;

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Spanning cell")
            && result.contains("First row content")
            && result.contains("Second line")
            && result.contains("Next row")
            && result.contains("More content"),
        "All rowspan content should be present: {result}"
    );
}

#[test]
fn test_table_colspan() {
    let html = r#"<table>
<tr><th colspan="2">Wide Header</th></tr>
<tr><td>Cell 1</td><td>Cell 2</td></tr>
</table>"#;

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Wide Header"), "Wide Header missing: {result}");
    assert!(result.contains("| Cell 1"), "Cell 1 missing: {result}");
    assert!(result.contains("| Cell 2"), "Cell 2 missing: {result}");
}

#[test]
fn test_table_cell_multiline_content() {
    // ~keep Test table cells with multiple divs (multiline content)
    // ~keep With br_in_tables enabled, divs should be separated by breaks
    let html = r"<table>
<tr><th>Header 1</th><th>Header 2</th></tr>
<tr><td>Cell 3</td><td>
    <div>Cell 4-1</div>
    <div>Cell 4-2</div>
</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Header 1")
            && result.contains("Header 2")
            && result.contains("Cell 3")
            && result.contains("Cell 4-1")
            && result.contains("Cell 4-2"),
        "All cell content should be present: {result}"
    );
}

#[test]
fn test_table_first_row_in_tbody_without_header() {
    let html = r"<table>
    <tbody>
        <tr><td>Cell 1</td><td>Cell 2</td></tr>
    </tbody>
    </table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Cell 1"), "Cell 1 missing: {result}");
    assert!(result.contains("| Cell 2"), "Cell 2 missing: {result}");
    assert!(result.contains("| ---"), "separator missing: {result}");
}

#[test]
fn test_tbody_only() {
    let html = "<table><tbody><tr><td>Data</td></tr></tbody></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("| Data |"));
}

#[test]
fn test_tfoot_basic() {
    let html = "<table><tfoot><tr><td>Footer</td></tr></tfoot><tbody><tr><td>Data</td></tr></tbody></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("| Footer"), "Footer cell missing: {result}");
    assert!(result.contains("| Data"), "Data cell missing: {result}");
}

#[test]
fn test_caption_with_formatting() {
    let html = r"<table><caption>Sales <strong>Report</strong> 2023</caption><tr><td>Data</td></tr></table>";
    let result = convert(html, None).unwrap();
    assert!(result.contains("*Sales **Report** 2023*"));
}

#[test]
fn test_table_with_links() {
    let html = r#"<table>
<tr><th>Name</th><th>Website</th></tr>
<tr><td>Example</td><td><a href="https://example.com">Link</a></td></tr>
</table>"#;

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Name"), "Name column missing: {result}");
    assert!(result.contains("| Website"), "Website column missing: {result}");
    assert!(result.contains("[Link](https://example.com)"), "link missing: {result}");
}

#[test]
fn test_table_with_code() {
    let html = r"<table>
<tr><th>Command</th></tr>
<tr><td><code>ls -la</code></td></tr>
</table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Command"), "Command column missing: {result}");
    assert!(result.contains("`ls -la`"), "code cell missing: {result}");
}

#[test]
fn test_table_empty_cells() {
    let html = r"<table>
<tr><td>Data</td><td></td></tr>
</table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Data |  |"));
}

#[test]
fn test_table_single_column() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Cell 1</td></tr>
<tr><td>Cell 2</td></tr>
</table>";

    let result = convert(html, None).unwrap();
    assert!(result.contains("| Header |"), "Header row missing: {result}");
    assert!(result.contains("| ---"), "separator missing: {result}");
    assert!(result.contains("| Cell 1 |"), "Cell 1 row missing: {result}");
    assert!(result.contains("| Cell 2 |"), "Cell 2 row missing: {result}");
}

#[test]
fn test_blogger_table_with_image() {
    let html = r#"
<table class="tr-caption-container">
  <a href="https://example.com/full-image.jpg">
    <img border="0" height="480"
         src="https://blogger.googleusercontent.com/img/test/IMG_0427.JPG"
         width="640" alt="Test Image" />
  </a>
</table>
"#;

    let result = convert(html, None).unwrap();

    assert!(
        result.contains("!["),
        "Result should contain markdown image syntax: {result}"
    );
    assert!(
        result.contains("blogger.googleusercontent.com"),
        "Result should contain image URL: {result}"
    );
    assert!(
        result.contains("example.com/full-image.jpg"),
        "Result should contain link URL: {result}"
    );
}

#[test]
fn test_table_with_image_no_rows() {
    let html = r#"<table><img src="https://example.com/image.jpg" alt="test image"></table>"#;
    let result = convert(html, None).unwrap();

    assert!(
        result.contains("![test image](https://example.com/image.jpg)"),
        "Image should be converted to markdown: {result}"
    );
}

#[test]
fn test_table_with_link_and_image_no_rows() {
    let html =
        r#"<table><a href="https://example.com"><img src="https://example.com/image.jpg" alt="test"></a></table>"#;
    let result = convert(html, None).unwrap();

    assert!(
        result.contains("[![test](https://example.com/image.jpg)](https://example.com)"),
        "Link-wrapped image should be converted to markdown: {result}"
    );
}

// ~keep ==============================================================================
// ~keep Comprehensive tests for <br> tags in table cells
// ~keep ==============================================================================
// ~keep These tests cover issue #429: with br_in_tables enabled, an explicit <br>
// ~keep inside a table cell must be preserved as a literal "<br>" so the GFM table
// ~keep row stays on one physical line. Emitting a hard break ("  \n" / "\\\n")
// ~keep injects a raw newline mid-row and produces an invalid table.

#[test]
fn test_br_in_table_cell_basic_spaces_style() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // ~keep br_in_tables keeps the cell on one physical line via a literal <br>.
    assert!(
        result.contains("Line 1<br>Line 2"),
        "Expected literal <br> inside the cell: {result}"
    );
    // ~keep A hard break must never split the row across physical lines.
    assert!(
        !result.contains("Line 1  \nLine 2"),
        "Must not emit a spaces-style hard break inside a cell: {result}"
    );
}

#[test]
fn test_br_in_table_cell_backslash_style() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        newline_style: html_to_markdown_rs::NewlineStyle::Backslash,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // ~keep br_in_tables emits a literal <br> regardless of newline_style.
    assert!(
        result.contains("Line 1<br>Line 2"),
        "Expected literal <br> inside the cell: {result}"
    );
    assert!(
        !result.contains("Line 1\\\nLine 2"),
        "Must not emit a backslash-style hard break inside a cell: {result}"
    );
}

#[test]
fn test_br_in_table_cell_issue_429_repro() {
    // ~keep Exact repro from issue #429.
    let html = "<table><tr><td>First line<br>Second line</td><td>Other</td></tr></table>";

    for style in [
        html_to_markdown_rs::NewlineStyle::Spaces,
        html_to_markdown_rs::NewlineStyle::Backslash,
    ] {
        let options = ConversionOptions {
            br_in_tables: true,
            compact_tables: true,
            newline_style: style,
            ..Default::default()
        };
        let result = convert(html, Some(options)).unwrap();
        assert!(
            result.contains("| First line<br>Second line | Other |"),
            "Expected single-line GFM row with literal <br> ({style:?}): {result}"
        );
    }
}

#[test]
fn test_br_in_table_cell_case_variations() {
    let test_cases = vec![
        ("<br>", "lowercase br", true),
        ("<BR>", "uppercase BR", false),
        ("<br/>", "self-closing lowercase", true),
        ("<BR/>", "self-closing uppercase", false),
        ("<br />", "self-closing with space", true),
        ("<BR />", "self-closing uppercase with space", false),
        ("<Br>", "mixed case Br", false),
        ("<bR />", "mixed case bR with space", false),
    ];

    for (html_br, case_name, should_work) in test_cases {
        let html = format!(
            r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1{html_br}Line 2</td></tr>
</table>"
        );

        let options = ConversionOptions {
            br_in_tables: true,
            ..Default::default()
        };
        let result = convert(&html, Some(options)).unwrap();

        if should_work {
            assert!(
                result.contains("Line 1") && result.contains("Line 2"),
                "Failed for {case_name}: Both lines should be in output: {result}"
            );
        } else {
            assert!(
                result.contains("Line 1"),
                "Failed for {case_name}: At least first line should be in output: {result}"
            );
        }
    }
}

#[test]
fn test_br_in_table_cell_with_consecutive_paragraphs() {
    // ~keep Consecutive paragraphs in table cells generate <br> separators.
    // ~keep EXPECTED: These should be converted to proper line breaks
    // ~keep ACTUAL BUG: Output as literal <br> tags in the markdown table
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>
    <p>First paragraph</p>
    <p>Second paragraph</p>
</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    // ~keep The content should be on separate lines in the table cell
    assert!(
        result.contains("First paragraph"),
        "Should contain first paragraph: {result}"
    );
    assert!(
        result.contains("Second paragraph"),
        "Should contain second paragraph: {result}"
    );
}

#[test]
fn test_br_in_table_cell_with_consecutive_divs() {
    // ~keep Consecutive divs in table cells also generate <br> separators
    // ~keep EXPECTED: Should convert to proper line breaks
    // ~keep ACTUAL BUG: Output as literal <br> tags in the markdown table
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>
    <div>First line</div>
    <div>Second line</div>
    <div>Third line</div>
</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(result.contains("First line"), "Should contain first line: {result}");
    assert!(result.contains("Second line"), "Should contain second line: {result}");
    assert!(result.contains("Third line"), "Should contain third line: {result}");
}

#[test]
fn test_br_in_table_cell_with_formatting() {
    // ~keep Test <br> tags between formatted text in table cells
    // ~keep EXPECTED: "**Text1**  \n**Text2**"
    // ~keep ACTUAL BUG: "**Text1**  <br>**Text2**"
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><b>Text1</b><br><b>Text2</b></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(result.contains("**Text1**"), "Expected first formatted text: {result}");
    assert!(result.contains("**Text2**"), "Expected second formatted text: {result}");
}

#[test]
fn test_br_in_table_cell_multiple_breaks() {
    // ~keep Test multiple <br> tags in the same table cell
    // ~keep EXPECTED: All breaks converted to proper Markdown line breaks
    // ~keep ACTUAL BUG: All breaks output as literal <br> tags
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2<br>Line 3<br>Line 4</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Line 1")
            && result.contains("Line 2")
            && result.contains("Line 3")
            && result.contains("Line 4"),
        "All lines should be in output: {result}"
    );
}

#[test]
fn test_br_in_table_cell_with_surrounding_text() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Before <b>middle<br>line</b> after</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Before")
            && result.contains("**middle")
            && result.contains("line**")
            && result.contains("after"),
        "Should contain all text parts: {result}"
    );
}

#[test]
fn test_multiple_cells_with_br_tags() {
    let html = r"<table>
<tr><th>Col1</th><th>Col2</th><th>Col3</th></tr>
<tr><td>A1<br>A2</td><td>B1<br>B2</td><td>C1<br>C2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("A1")
            && result.contains("A2")
            && result.contains("B1")
            && result.contains("B2")
            && result.contains("C1")
            && result.contains("C2"),
        "All cell contents should be in output: {result}"
    );
}

#[test]
fn test_br_in_header_and_data_cells() {
    let html = r"<table>
<tr><th>Header1<br>Line2</th><th>Header3</th></tr>
<tr><td>Data1<br>Line2</td><td>Data3</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Header1")
            && result.contains("Line2")
            && result.contains("Header3")
            && result.contains("Data1")
            && result.contains("Data3"),
        "All cell contents should be in output: {result}"
    );
}

#[test]
fn test_br_in_nested_formatting_in_table_cell() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><strong><em>Bold italic<br>next line</em></strong></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Bold italic") && result.contains("next line"),
        "Nested formatting content should be preserved: {result}"
    );
}

#[test]
fn test_br_in_table_cell_with_link() {
    // ~keep Test <br> tags between links in table cells
    // ~keep EXPECTED: "[Link1](url)  \n[Link2](url)"
    // ~keep ACTUAL BUG: "[Link1](url)  <br>[Link2](url)"
    let html = r#"<table>
<tr><th>Header</th></tr>
<tr><td><a href="https://example.com">Link1</a><br><a href="https://example.org">Link2</a></td></tr>
</table>"#;

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Link1")
            && result.contains("example.com")
            && result.contains("Link2")
            && result.contains("example.org"),
        "Links should be preserved: {result}"
    );
}

#[test]
fn test_br_with_no_br_in_tables_option() {
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td>Line 1<br>Line 2</td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: false,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("Line 1") && result.contains("Line 2"),
        "Both lines should appear in output: {result}"
    );
}

#[test]
fn test_br_in_table_with_code_in_cell() {
    // ~keep Test <br> tags between code elements in table cells
    // ~keep EXPECTED: "`command1`  \n`command2`"
    // ~keep ACTUAL BUG: "`command1`  <br>`command2`"
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><code>command1</code><br><code>command2</code></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("command1") && result.contains("command2"),
        "Code blocks should be preserved: {result}"
    );
}

#[test]
fn test_br_in_table_empty_cell_with_break() {
    // ~keep Test <br> tag as sole content of table cell
    // ~keep EXPECTED: Cell should be empty or have proper line break representation
    // ~keep ACTUAL BUG: May output literal <br> tag
    let html = r"<table>
<tr><th>Header</th></tr>
<tr><td><br></td></tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains('|'),
        "Should still generate valid table structure: {result}"
    );
}

#[test]
fn test_br_in_table_with_mixed_content() {
    let html = r"<table>
<tr><th>Status</th><th>Description</th></tr>
<tr>
    <td>Active</td>
    <td>First step<br><strong>Bold text</strong><br>Final step</td>
</tr>
</table>";

    let options = ConversionOptions {
        br_in_tables: true,
        ..Default::default()
    };
    let result = convert(html, Some(options)).unwrap();

    assert!(
        result.contains("First step")
            && result.contains("**Bold text**")
            && result.contains("Final step")
            && result.contains("Active"),
        "Should contain all content: {result}"
    );
}

#[test]
fn test_table_colspan_no_header_issue_233() {
    let html = r#"<table>
      <tr>
        <td colspan="2">Cell spanning 2 columns</td>
      </tr>
      <tr>
        <td>Cell 1</td>
        <td>Cell 2</td>
      </tr>
    </table>"#;
    let result = html_to_markdown_rs::convert(html, None)
        .unwrap()
        .content
        .unwrap_or_default();
    assert!(
        result.contains("| Cell spanning 2 columns"),
        "spanning cell missing: {result}"
    );
    assert!(result.contains("| Cell 1"), "Cell 1 missing: {result}");
    assert!(result.contains("| Cell 2"), "Cell 2 missing: {result}");
}

/// Regression test for issue #13: a data row with MORE cells than the header row used to
/// render with a separator row sized to the header's (narrower) column count. GFM-compliant
/// renderers silently drop any cell past the declared column count, so the third cell's
/// content ("3") would vanish even though our own Markdown text technically still contained
/// it. The header/separator must now widen to the table's true max column count.
#[test]
fn should_widen_separator_row_when_a_data_row_has_more_cells_than_the_header() {
    let html = "<table><tr><th>A</th><th>B</th></tr><tr><td>1</td><td>2</td><td>3</td></tr></table>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "| A | B |   |\n| --- | --- | --- |\n| 1 | 2 | 3 |\n");
}

/// Regression test for issue #13: a data row with FEWER cells than the header must still be
/// explicitly padded to the header's column count, keeping every row's declared column count
/// consistent (defensive against non-GFM-strict Markdown table parsers).
#[test]
fn should_pad_ragged_row_with_fewer_cells_than_the_header() {
    let html = "<table><tr><th>A</th><th>B</th><th>C</th></tr><tr><td>1</td><td>2</td></tr></table>";
    let result = convert(html, None).unwrap();
    assert_eq!(result, "| A | B | C |\n| --- | --- | --- |\n| 1 | 2 |   |\n");
}

/// Regression test for issue #13: `scan_table_node`'s nested-table/row-count scan did not
/// stop at nested `<table>` boundaries, so a straight chain of one-nested-table-per-cell
/// tables accumulated a `nested_table_count` proportional to the remaining chain depth at
/// every level. That misfired the `looks_like_layout` heuristic (`nested_table_count > 1`)
/// for every table except the innermost couple, converting real nested-table structure into
/// a broken mix of list bullets and stray pipe/dash table syntax. A three-level chain (well
/// past the `> 1` threshold under the old counting) must still render as clean pipe tables.
#[test]
fn should_not_misclassify_a_deep_nested_table_chain_as_a_layout_table() {
    let html =
        "<table><tr><td><table><tr><td><table><tr><td>leaf</td></tr></table></td></tr></table></td></tr></table>";
    let result = convert(html, None).unwrap();
    assert!(
        !result.trim_start().starts_with('-'),
        "a straight nested-table chain must not be misclassified as a layout list: {result}"
    );
    assert!(result.contains("leaf"), "leaf cell content must survive: {result}");
    assert!(
        result.matches("| ---").count() >= 1,
        "expected pipe-table separator syntax, got: {result}"
    );
}
