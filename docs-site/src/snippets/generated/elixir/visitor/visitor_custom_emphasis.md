```elixir title="Elixir"
visitor = %{
      :handle_emphasis => fn(args) ->
        {:custom, ">>>" <> Map.get(args, "text", "") <> "<<<"}
      end,
    }

result = HtmlToMarkdown.convert("<p>This is <em>important</em> text.</p>", visitor)

```
