require "./spec_helper"

describe HtmlToMarkdown do
  describe "real-world" do
    it "Blog post with headings, paragraphs, code blocks, and links converts to readable Markdown" do
      __result = HtmlToMarkdown.convert("<article><h1>Getting Started with Rust</h1><p>Rust is a systems programming language focused on <strong>safety</strong>, <em>performance</em>, and concurrency. It was created by <a href=\"https://www.mozilla.org\">Mozilla</a> and has grown significantly in popularity.</p><h2>Installation</h2><p>Install Rust using the official installer:</p><pre><code class=\"language-bash\">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre><h2>Hello World</h2><p>Create your first Rust program:</p><pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello, world!\");\n}</code></pre><p>Run it with <code>cargo run</code> from your project directory.</p><h2>Key Concepts</h2><ul><li>Ownership and borrowing</li><li>Lifetimes</li><li>Traits and generics</li><li>Pattern matching</li></ul><p>For more information, visit the <a href=\"https://doc.rust-lang.org/book/\">Rust Book</a>.</p></article>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("# Getting Started with Rust")
      __result.content.to_s.should contain("## Installation")
      __result.content.to_s.should contain("## Hello World")
      __result.content.to_s.should contain("## Key Concepts")
      __result.content.to_s.should contain("cargo run")
      __result.content.to_s.should contain("[Mozilla](https://www.mozilla.org)")
      __result.content.to_s.should contain("- Ownership and borrowing")
    end
    it "Documentation page with nested lists, code examples, and blockquotes converts correctly" do
      __result = HtmlToMarkdown.convert("<div class=\"docs\"><h1>API Reference</h1><p>This guide covers the core API for the <code>html-to-markdown</code> library.</p><blockquote><p><strong>Note:</strong> All functions are thread-safe and can be called from multiple threads concurrently.</p></blockquote><h2>convert_html</h2><p>Converts an HTML string to Markdown format.</p><pre><code class=\"language-rust\">pub fn convert_html(html: &amp;str) -&gt; Result&lt;String, ConversionError&gt;</code></pre><h3>Parameters</h3><ul><li><code>html</code> - The HTML input string<ul><li>Must be valid UTF-8</li><li>Maximum size: 50MB</li></ul></li></ul><h3>Returns</h3><ul><li><code>Ok(String)</code> - The converted Markdown</li><li><code>Err(ConversionError)</code> - If conversion fails</li></ul><h3>Example</h3><pre><code class=\"language-rust\">let markdown = convert_html(\"&lt;h1&gt;Hello&lt;/h1&gt;\").unwrap();\nassert_eq!(markdown, \"# Hello\");</code></pre><h2>ConversionOptions</h2><p>Configure conversion behavior using the builder pattern:</p><pre><code class=\"language-rust\">let options = ConversionOptions::builder()\n    .heading_style(HeadingStyle::ATX)\n    .code_block_style(CodeBlockStyle::Fenced)\n    .build();</code></pre><blockquote><p>See the <a href=\"/docs/options\">options reference</a> for a full list of configuration values.</p></blockquote></div>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("# API Reference")
      __result.content.to_s.should contain("## convert_html")
      __result.content.to_s.should contain("### Parameters")
      __result.content.to_s.should contain("### Returns")
      __result.content.to_s.should contain("### Example")
      __result.content.to_s.should contain("## ConversionOptions")
      __result.content.to_s.should contain("> ")
      __result.content.to_s.should contain("thread-safe")
      __result.content.to_s.should contain("convert_html")
      __result.content.to_s.should contain("ConversionOptions")
    end
    it "Product page with table, images, and lists converts correctly" do
      __result = HtmlToMarkdown.convert("<div class=\"product\"><h1>Wireless Keyboard Pro</h1><img src=\"https://example.com/keyboard.jpg\" alt=\"Wireless Keyboard Pro\"><p>The ultimate wireless keyboard for professionals. Features a comfortable layout with <strong>backlit keys</strong> and <em>ultra-long battery life</em>.</p><h2>Specifications</h2><table><thead><tr><th>Feature</th><th>Details</th></tr></thead><tbody><tr><td>Battery Life</td><td>Up to 12 months</td></tr><tr><td>Connectivity</td><td>Bluetooth 5.0</td></tr><tr><td>Key Travel</td><td>2mm</td></tr><tr><td>Weight</td><td>750g</td></tr></tbody></table><h2>What's in the Box</h2><ul><li>Wireless Keyboard Pro</li><li>USB-C charging cable</li><li>USB receiver dongle</li><li>Quick start guide</li></ul><h2>Compatibility</h2><p>Compatible with <strong>Windows</strong>, <strong>macOS</strong>, <strong>Linux</strong>, <strong>iOS</strong>, and <strong>Android</strong>.</p></div>", nil)
      __result.content.to_s.should_not be_empty
      __result.content.to_s.should contain("# Wireless Keyboard Pro")
      __result.content.to_s.should contain("![Wireless Keyboard Pro](https://example.com/keyboard.jpg)")
      __result.content.to_s.should contain("## Specifications")
      __result.content.to_s.should contain("Battery Life")
      __result.content.to_s.should contain("12 months")
      __result.content.to_s.should contain("Bluetooth 5.0")
      __result.content.to_s.should contain("## What's in the Box")
      __result.content.to_s.should contain("USB-C charging cable")
      __result.content.to_s.should contain("|")
      __result.content.to_s.should contain("---")
    end
  end
end
