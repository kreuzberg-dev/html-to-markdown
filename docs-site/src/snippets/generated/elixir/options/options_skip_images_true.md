```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{skip_images: true}
result = HtmlToMarkdown.convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", options_value)

```
