```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".nav"], output_format: "Plain"}
result = HtmlToMarkdown.convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options_value)

```
