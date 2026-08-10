```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_block_style: "Indented"}
result = HtmlToMarkdown.convert("<pre><code>print('hello')</code></pre>", options_value)

```
