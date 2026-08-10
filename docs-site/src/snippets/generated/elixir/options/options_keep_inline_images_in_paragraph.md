```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{keep_inline_images_in: ["p"]}
result = HtmlToMarkdown.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options_value)

```
