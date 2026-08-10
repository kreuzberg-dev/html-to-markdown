```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Backticks"}
result = HtmlToMarkdown.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options_value)

```
