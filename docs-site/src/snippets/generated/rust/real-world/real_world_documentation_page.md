---
id: fixture_rust_real_world_documentation_page
language: rust
target: rust
level: typecheck
requires: []
side_effect: safe
---

```rust title="Rust"
use html_to_markdown_rs::convert;
use html_to_markdown_rs::ConversionOptions;

fn main() {
    let html = r##"<div class="docs"><h1>API Reference</h1><p>This guide covers the core API for the <code>html-to-markdown</code> library.</p><blockquote><p><strong>Note:</strong> All functions are thread-safe and can be called from multiple threads concurrently.</p></blockquote><h2>convert_html</h2><p>Converts an HTML string to Markdown format.</p><pre><code class="language-rust">pub fn convert_html(html: &amp;str) -&gt; Result&lt;String, ConversionError&gt;</code></pre><h3>Parameters</h3><ul><li><code>html</code> - The HTML input string<ul><li>Must be valid UTF-8</li><li>Maximum size: 50MB</li></ul></li></ul><h3>Returns</h3><ul><li><code>Ok(String)</code> - The converted Markdown</li><li><code>Err(ConversionError)</code> - If conversion fails</li></ul><h3>Example</h3><pre><code class="language-rust">let markdown = convert_html("&lt;h1&gt;Hello&lt;/h1&gt;").unwrap();
    .heading_style(HeadingStyle::ATX)
    .code_block_style(CodeBlockStyle::Fenced)
    .build();</code></pre><blockquote><p>See the <a href="/docs/options">options reference</a> for a full list of configuration values.</p></blockquote></div>"##;
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
