```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{default_title: true}
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Link</a></p>", options_value)

```
