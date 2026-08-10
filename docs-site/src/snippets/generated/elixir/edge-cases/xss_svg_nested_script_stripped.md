```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Before SVG.</p><svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert('svg-xss')</script><text>SVG text</text></svg><p>After SVG.</p>")

```
