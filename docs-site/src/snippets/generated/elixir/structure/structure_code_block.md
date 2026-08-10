```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{include_document_structure: true}
result = HtmlToMarkdown.convert("<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", options_value)

```
