```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{url_escape_style: "percent"}
result = HtmlToMarkdown.convert("<a href=\"/file (1) <draft>.pdf\">file</a>", options_value)

```
