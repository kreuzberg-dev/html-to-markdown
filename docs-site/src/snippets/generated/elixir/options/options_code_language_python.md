```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{code_language: "python"}
result = HtmlToMarkdown.convert("<pre><code>def hello(): pass</code></pre>", options_value)

```
