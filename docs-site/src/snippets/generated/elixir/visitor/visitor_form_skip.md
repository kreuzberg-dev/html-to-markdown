---
id: fixture_elixir_visitor_form_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_form => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", visitor)

```
