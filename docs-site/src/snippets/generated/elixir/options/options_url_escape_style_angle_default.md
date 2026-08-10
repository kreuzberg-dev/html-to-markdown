```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{url_escape_style: "angle"}
result = HtmlToMarkdown.convert("<a href=\"/file (1).pdf\">file</a>", options_value)

```
