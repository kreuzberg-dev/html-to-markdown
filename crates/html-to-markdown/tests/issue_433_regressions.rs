// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]
#![cfg(feature = "inline-images")]

use html_to_markdown_rs::{ConversionOptions, TierStrategy, convert};

fn content(html: &str, options: ConversionOptions) -> String {
    convert(html, Some(options)).unwrap().content.unwrap_or_default()
}

#[test]
fn keep_inline_images_in_td_survives_tier2_layout_cell_issue_433() {
    // ~keep Rows with differing cell counts classify this as a layout table
    // ~keep (email-signature shape), so cells are converted as inline — the exact
    // ~keep path issue #433 hits, where the image would otherwise drop to alt text.
    let html = r#"
<table role="presentation">
  <tr><td><p><span><img src="image.png" alt="Image"></span></p></td></tr>
  <tr><td>Name</td><td>Title</td></tr>
</table>
"#;

    let options = ConversionOptions {
        keep_inline_images_in: vec!["td".to_string(), "th".to_string()],
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };

    let result = content(html, options);
    assert!(
        result.contains("![Image](image.png)"),
        "Image in a td listed in keep_inline_images_in must stay markdown: {result:?}"
    );
}

#[test]
fn keep_inline_images_in_excluding_cell_reduces_to_alt_issue_433() {
    // ~keep A non-empty keep list that does NOT include the cell tag still reduces
    // ~keep the layout-cell image to alt text (default behavior preserved).
    let html = r#"
<table role="presentation">
  <tr><td><p><span><img src="image.png" alt="Image"></span></p></td></tr>
  <tr><td>Name</td><td>Title</td></tr>
</table>
"#;

    let options = ConversionOptions {
        keep_inline_images_in: vec!["h1".to_string()], // ~keep excludes td/th
        tier_strategy: TierStrategy::Tier2,
        ..ConversionOptions::default()
    };

    let result = content(html, options);
    assert!(
        !result.contains("![Image](image.png)"),
        "A keep list excluding the cell tag should reduce to alt text: {result:?}"
    );
    assert!(result.contains("Image"), "{result:?}");
}
