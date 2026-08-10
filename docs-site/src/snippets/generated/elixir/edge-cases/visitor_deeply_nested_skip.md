```elixir title="Elixir"
visitor = %{
      :handle_mark => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", visitor)

```
