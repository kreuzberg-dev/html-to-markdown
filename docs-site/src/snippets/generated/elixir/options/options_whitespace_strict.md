```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{whitespace_mode: "Strict"}
result = HtmlToMarkdown.convert("<p>Preserved   spacing.</p>", options_value)

```
