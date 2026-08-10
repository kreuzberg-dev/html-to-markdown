```elixir title="Elixir"
visitor = %{
      :handle_input => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", visitor)

```
