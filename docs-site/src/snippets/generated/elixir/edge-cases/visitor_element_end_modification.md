---
id: fixture_elixir_visitor_element_end_modification
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_element_end => fn(_args) ->
        {:custom, "MODIFIED OUTPUT"}
      end,
    }

result = HtmlToMarkdown.convert("<blockquote><p>Original quote</p></blockquote>", visitor)

```
