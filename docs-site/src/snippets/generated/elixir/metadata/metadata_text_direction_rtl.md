```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<html lang=\"ar\" dir=\"rtl\"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>", options_value)

```
