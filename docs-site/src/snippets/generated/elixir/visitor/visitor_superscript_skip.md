---
id: fixture_elixir_visitor_superscript_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_superscript => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>", visitor)

```
