---
id: fixture_elixir_visitor_skip_strong
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_strong => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Normal <strong>bold text</strong> normal</p>", visitor)

```
