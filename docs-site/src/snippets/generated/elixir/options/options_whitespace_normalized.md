```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{whitespace_mode: "Normalized"}
result = HtmlToMarkdown.convert("<p>Text   with    extra   spaces.</p>", options_value)

```
