```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Tildes"}
result = HtmlToMarkdown.convert("<pre><code>some code</code></pre>", options_value)

```
