```elixir
opts = %HtmlToMarkdown.ConversionOptions{wrap: true, wrap_width: 40}
{:ok, result} = HtmlToMarkdown.convert("<h1>Hello</h1><p>World</p>", opts)
IO.puts(result.content)
```
