---
id: fixture_elixir_visitor_subscript_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_subscript => fn(args) ->
        {:custom, "~" <> Map.get(args, "text", "") <> "~"}
      end,
    }

result = HtmlToMarkdown.convert("<p>H<sub>2</sub>O is water.</p>", visitor)

```
