```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p>Jump to <a href=\"\#section\">section</a> below.</p>", options_value)

```
