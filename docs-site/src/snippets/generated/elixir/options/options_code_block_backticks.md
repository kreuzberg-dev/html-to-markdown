```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Backticks"}
result = HtmlToMarkdown.convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", options_value)

```
