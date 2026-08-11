---
id: fixture_elixir_visitor_line_break_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_line_break => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", visitor)

```
