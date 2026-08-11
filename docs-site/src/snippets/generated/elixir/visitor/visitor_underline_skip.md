---
id: fixture_elixir_visitor_underline_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_underline => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Normal text with <u>underlined part</u> and more text.</p>", visitor)

```
