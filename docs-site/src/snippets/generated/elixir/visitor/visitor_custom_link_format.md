---
id: fixture_elixir_visitor_custom_link_format
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_link => fn(args) ->
        {:custom, Map.get(args, "text", "") <> " (" <> Map.get(args, "href", "") <> ")"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", visitor)

```
