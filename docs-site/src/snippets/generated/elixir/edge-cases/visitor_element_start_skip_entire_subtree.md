---
id: fixture_elixir_visitor_element_start_skip_entire_subtree
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_element_start => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<div><h1>Title</h1><p>Content</p></div>", visitor)

```
