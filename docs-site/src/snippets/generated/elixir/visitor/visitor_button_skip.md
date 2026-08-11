---
id: fixture_elixir_visitor_button_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_button => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", visitor)

```
