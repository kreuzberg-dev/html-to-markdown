```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{strip_tags: ["div", "span"]}
result = HtmlToMarkdown.convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", options_value)

```
