---
id: fixture_elixir_visitor_iframe_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_iframe => fn(_args) ->
        {:custom, "[EMBEDDED: https://maps.example.com/embed]"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", visitor)

```
