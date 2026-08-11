---
id: fixture_elixir_visitor_preserve_html
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_custom_element => fn(_args) ->
        :preserve_html
      end,
    }

result = HtmlToMarkdown.convert("<div><custom-tag>Custom content</custom-tag></div>", visitor)

```
