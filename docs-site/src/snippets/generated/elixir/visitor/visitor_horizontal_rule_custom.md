```elixir title="Elixir"
visitor = %{
      :handle_horizontal_rule => fn(_args) ->
        {:custom, "\n[DIVIDER]\n"}
      end,
    }

result = HtmlToMarkdown.convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", visitor)

```
