```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options_value)

```
