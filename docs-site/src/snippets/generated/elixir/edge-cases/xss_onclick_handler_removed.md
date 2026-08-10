```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p><a href=\"https://example.com\" onclick=\"alert('xss')\">Click me</a></p><button onmouseover=\"steal_data()\">Hover me</button>")

```
