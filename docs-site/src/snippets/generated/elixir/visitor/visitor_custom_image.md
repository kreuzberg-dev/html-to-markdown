---
id: fixture_elixir_visitor_custom_image
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_image => fn(args) ->
        {:custom, "[Image: " <> Map.get(args, "alt", "") <> "]"}
      end,
    }

result = HtmlToMarkdown.convert("<img src=\"banner.png\" alt=\"Banner\">", visitor)

```
