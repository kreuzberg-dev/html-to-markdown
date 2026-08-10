```elixir title="Elixir"
visitor = %{
      :handle_mark => fn(args) ->
        {:custom, "==" <> Map.get(args, "text", "") <> "=="}
      end,
    }

result = HtmlToMarkdown.convert("<p>This is a <mark>highlighted passage</mark> in the text.</p>", visitor)

```
