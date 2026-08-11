---
id: fixture_rust_real_world_blog_post
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
    let html = r#"<article><h1>Getting Started with Rust</h1><p>Rust is a systems programming language focused on <strong>safety</strong>, <em>performance</em>, and concurrency. It was created by <a href="https://www.mozilla.org">Mozilla</a> and has grown significantly in popularity.</p><h2>Installation</h2><p>Install Rust using the official installer:</p><pre><code class="language-bash">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre><h2>Hello World</h2><p>Create your first Rust program:</p><pre><code class="language-rust">fn main() {
    println!("Hello, world!");
    let options: ConversionOptions = Default::default();
    let _ = convert(html, Some(options.clone()));
}

```
