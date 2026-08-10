```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{strip_newlines: true}
result = HtmlToMarkdown.convert("<p>First paragraph.</p><p>Second paragraph.</p>", options_value)

```
