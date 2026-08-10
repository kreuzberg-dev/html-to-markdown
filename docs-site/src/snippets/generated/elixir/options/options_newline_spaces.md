```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{newline_style: "Spaces"}
result = HtmlToMarkdown.convert("<p>First<br>Second</p>", options_value)

```
