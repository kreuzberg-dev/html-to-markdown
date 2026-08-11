---
id: fixture_elixir_visitor_skip_heading
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_heading => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<h1>Title</h1><p>Body text remains.</p>", visitor)

```
