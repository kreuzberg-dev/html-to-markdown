```elixir title="Elixir"
visitor = %{
      :handle_mark => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", visitor)

```
