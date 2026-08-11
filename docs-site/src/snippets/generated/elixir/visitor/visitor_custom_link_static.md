---
id: fixture_elixir_visitor_custom_link_static
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_link => fn(_args) ->
        {:custom, "[REDACTED LINK]"}
      end,
    }

result = HtmlToMarkdown.convert("<a href=\"https://example.com\">Click here</a>", visitor)

```
