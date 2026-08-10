```elixir title="Elixir"
visitor = %{
      :handle_subscript => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>", visitor)

```
