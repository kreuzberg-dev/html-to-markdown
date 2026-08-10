```elixir title="Elixir"
visitor = %{
      :handle_form => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", visitor)

```
