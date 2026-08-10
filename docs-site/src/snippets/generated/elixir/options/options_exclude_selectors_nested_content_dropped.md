```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".sidebar"]}
result = HtmlToMarkdown.convert("<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", options_value)

```
