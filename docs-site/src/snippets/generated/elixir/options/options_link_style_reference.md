```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{link_style: "Reference"}
result = HtmlToMarkdown.convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options_value)

```
