```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".cookie-banner"]}
result = HtmlToMarkdown.convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options_value)

```
