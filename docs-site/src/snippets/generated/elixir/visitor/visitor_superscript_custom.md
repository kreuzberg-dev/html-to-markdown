---
id: fixture_elixir_visitor_superscript_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_superscript => fn(args) ->
        {:custom, "^" <> Map.get(args, "text", "") <> "^"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", visitor)

```
