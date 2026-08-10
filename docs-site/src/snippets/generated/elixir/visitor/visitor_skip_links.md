```elixir title="Elixir"
visitor = %{
      :handle_link => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", visitor)

```
