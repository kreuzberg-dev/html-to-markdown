---
id: fixture_elixir_visitor_input_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_input => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", visitor)

```
