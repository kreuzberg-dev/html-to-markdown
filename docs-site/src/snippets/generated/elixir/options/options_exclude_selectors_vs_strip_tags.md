```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".wrapper"]}
result = HtmlToMarkdown.convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options_value)

```
