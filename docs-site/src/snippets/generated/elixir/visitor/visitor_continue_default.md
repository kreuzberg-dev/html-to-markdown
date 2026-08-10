```elixir title="Elixir"
visitor = %{
      :handle_strong => fn(_args) ->
        :continue
      end,
    }

result = HtmlToMarkdown.convert("<p>Hello <strong>World</strong></p>", visitor)

```
