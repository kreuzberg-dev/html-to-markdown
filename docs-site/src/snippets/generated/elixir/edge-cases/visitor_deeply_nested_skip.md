---
id: fixture_elixir_visitor_deeply_nested_skip
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

result = HtmlToMarkdown.convert("<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", visitor)

```
