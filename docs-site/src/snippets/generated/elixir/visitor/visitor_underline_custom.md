---
id: fixture_elixir_visitor_underline_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_underline => fn(args) ->
        {:custom, "_" <> Map.get(args, "text", "") <> "_"}
      end,
    }

result = HtmlToMarkdown.convert("<p>This is <u>very important</u> text.</p>", visitor)

```
