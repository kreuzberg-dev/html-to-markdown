---
id: fixture_elixir_visitor_image_bare_string_preserves_case
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_image => fn(args) ->
        {:custom, "[image: " <> Map.get(args, "alt", "") <> " -> " <> Map.get(args, "src", "") <> "]"}
      end,
    }

result = HtmlToMarkdown.convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", visitor)

```
