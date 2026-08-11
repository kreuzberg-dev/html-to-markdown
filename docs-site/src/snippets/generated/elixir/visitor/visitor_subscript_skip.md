---
id: fixture_elixir_visitor_subscript_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_subscript => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>", visitor)

```
