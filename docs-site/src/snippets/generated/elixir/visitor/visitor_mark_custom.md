---
id: fixture_elixir_visitor_mark_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_mark => fn(args) ->
        {:custom, "==" <> Map.get(args, "text", "") <> "=="}
      end,
    }

result = HtmlToMarkdown.convert("<p>This is a <mark>highlighted passage</mark> in the text.</p>", visitor)

```
