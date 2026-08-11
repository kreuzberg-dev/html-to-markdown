---
id: fixture_elixir_visitor_custom_heading
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_heading => fn(args) ->
        {:custom, "--- " <> Map.get(args, "text", "") <> " ---"}
      end,
    }

result = HtmlToMarkdown.convert("<h2>Section Title</h2><p>Content below heading.</p>", visitor)

```
