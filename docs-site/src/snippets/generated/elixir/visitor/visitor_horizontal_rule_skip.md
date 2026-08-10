```elixir title="Elixir"
visitor = %{
      :handle_horizontal_rule => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", visitor)

```
