```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_images: false}
result = HtmlToMarkdown.convert("<p>Text with <img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8/5+hHgAHggJ/PchI7wAAAABJRU5ErkJggg==\" alt=\"pixel\"> image.</p>", options_value)

```
