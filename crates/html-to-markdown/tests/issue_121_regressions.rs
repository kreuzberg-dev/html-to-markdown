// ~keep Rust inner attributes below are crate-level attributes, not a shell shebang.
#![allow(missing_docs)]

fn convert(
    html: &str,
    opts: Option<html_to_markdown_rs::ConversionOptions>,
) -> html_to_markdown_rs::error::Result<String> {
    html_to_markdown_rs::convert(html, opts).map(|r| r.content.unwrap_or_default())
}

use std::fs;
use std::path::PathBuf;

use html_to_markdown_rs::ConversionOptions;

fn fixture_path(name: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "../../test_documents/html/issues", name]
        .iter()
        .collect()
}

fn default_options() -> ConversionOptions {
    ConversionOptions {
        extract_metadata: false,
        autolinks: false,
        ..Default::default()
    }
}

fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

// ~keep The first `<li>` wraps its nested `<ul>` in a `<div class="children-container">`,
// ~keep which forces our own rendering to add a blank-line separator around that item (a bare
// ~keep `<div>` can't just run into the next `- ` marker). That blank line makes the WHOLE
// ~keep list loose on any CommonMark-compliant reparse (looseness is a per-list, not
// ~keep per-item-pair, property), so the fixture's remaining plain items must carry the same
// ~keep blank-line separation to reach a fixpoint -- confirmed by rendering the old fixture's
// ~keep markdown back to HTML and reconverting it, which produced exactly this fully-loose
// ~keep form instead of reproducing the old fixture (see the `commonmark_spec_fixpoint`
// ~keep oracle this mirrors).
#[test]
fn converts_spa_menu_fixture() {
    let html = fs::read_to_string(fixture_path("gh-121-spa-app.html")).expect("read spa html");
    let expected = fs::read_to_string(fixture_path("gh-121-spa-app.md")).expect("read spa markdown");

    let result = convert(&html, Some(default_options())).expect("convert spa html");
    assert_eq!(normalize_newlines(&result), normalize_newlines(&expected));
}

#[test]
fn converts_hacker_news_fixture() {
    let html = fs::read_to_string(fixture_path("gh-121-hacker-news.html")).expect("read hn html");
    let expected = fs::read_to_string(fixture_path("gh-121-hacker-news.md")).expect("read hn markdown");

    let result = convert(&html, Some(default_options())).expect("convert hn html");
    assert_eq!(normalize_newlines(&result), normalize_newlines(&expected));
}
