```elixir title="Elixir"
visitor = %{
      :handle_underline => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Normal text with <u>underlined part</u> and more text.</p>", visitor)

```
