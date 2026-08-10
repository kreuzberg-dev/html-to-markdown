```elixir title="Elixir"
visitor = %{
      :handle_strong => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Normal <strong>bold text</strong> normal</p>", visitor)

```
