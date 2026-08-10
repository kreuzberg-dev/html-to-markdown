```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{capture_svg: false, extract_images: true}
result = HtmlToMarkdown.convert("<p>Below SVG:</p><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"><rect width=\"10\" height=\"10\" fill=\"red\"/></svg>", options_value)

```
