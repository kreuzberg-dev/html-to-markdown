---
id: fixture_elixir_visitor_line_break_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_line_break => fn(_args) ->
        {:custom, " | "}
      end,
    }

result = HtmlToMarkdown.convert("<p>First line<br>Second line<br>Third line</p>", visitor)

```
