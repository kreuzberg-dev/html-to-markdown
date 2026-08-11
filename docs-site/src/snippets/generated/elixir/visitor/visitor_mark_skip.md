---
id: fixture_elixir_visitor_mark_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_mark => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", visitor)

```
