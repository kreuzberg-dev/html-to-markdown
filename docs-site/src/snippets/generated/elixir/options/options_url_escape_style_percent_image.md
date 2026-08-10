```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{url_escape_style: "percent"}
result = HtmlToMarkdown.convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options_value)

```
