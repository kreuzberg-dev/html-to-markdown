```elixir
# Binary data (detected via magic bytes) is rejected before parsing.
html = "%PDF-1.4 not actually HTML"

case HtmlToMarkdown.convert(html) do
  {:ok, result} ->
    IO.puts(result.content)

  {:error, message} ->
    IO.puts(:stderr, "conversion failed: #{message}")
end
```
