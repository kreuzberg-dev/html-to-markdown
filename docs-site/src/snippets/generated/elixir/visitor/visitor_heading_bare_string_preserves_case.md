---
id: fixture_elixir_visitor_heading_bare_string_preserves_case
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_heading => fn(args) ->
        {:custom, "\#\# " <> Map.get(args, "text", "") <> " \#\#"}
      end,
    }

result = HtmlToMarkdown.convert("<h2>Important Section Title</h2><p>Body.</p>", visitor)

```
