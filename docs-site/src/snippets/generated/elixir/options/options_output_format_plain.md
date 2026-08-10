```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{output_format: "Plain"}
result = HtmlToMarkdown.convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options_value)

```
