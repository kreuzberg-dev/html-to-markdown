```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{heading_style: "Atx", output_format: "Markdown"}
result = HtmlToMarkdown.convert("<h1>Title</h1><p>Some text.</p>", options_value)

```
