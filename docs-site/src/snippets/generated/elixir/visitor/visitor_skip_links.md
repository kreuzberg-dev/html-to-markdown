---
id: fixture_elixir_visitor_skip_links
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_link => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", visitor)

```
