```elixir title="Elixir"
visitor = %{
      :handle_subscript => fn(args) ->
        {:custom, "~" <> Map.get(args, "text", "") <> "~"}
      end,
    }

result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O is water.</p>", visitor)

```
