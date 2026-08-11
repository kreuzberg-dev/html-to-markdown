---
id: fixture_elixir_visitor_continue_default
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_strong => fn(_args) ->
        :continue
      end,
    }

result = HtmlToMarkdown.convert("<p>Hello <strong>World</strong></p>", visitor)

```
