```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{heading_style: "Atx"}
result = HtmlToMarkdown.convert("<h1>Title</h1><h2>Subtitle</h2>", options_value)

```
