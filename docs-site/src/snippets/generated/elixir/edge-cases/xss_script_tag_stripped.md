```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>")

```
