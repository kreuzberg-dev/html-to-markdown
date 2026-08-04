```rust
use html_to_markdown_rs::{ConversionOptions, convert};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = r#"
<table>
    <tr><th>Name</th><th>Age</th></tr>
    <tr><td>Alice</td><td>30</td></tr>
    <tr><td>Bob</td><td>25</td></tr>
</table>
"#;

    // `tables` is collected alongside the document tree, so it must be enabled.
    let options = ConversionOptions::builder().include_document_structure(true).build();
    let result = convert(html, Some(options))?;

    for table in &result.tables {
        for cell in &table.grid.cells {
            let kind = if cell.is_header { "Header" } else { "Cell" };
            println!("  {kind} (r{},c{}): {}", cell.row, cell.col, cell.content);
        }
    }
    Ok(())
}
```
