```elixir title="Elixir"
visitor = %{
      :handle_link => fn(args) ->
        {:custom, "[" <> Map.get(args, "text", "") <> "](https://new-cdn.com/file.pdf)"}
      end,
    }

result = HtmlToMarkdown.convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", visitor)

```
