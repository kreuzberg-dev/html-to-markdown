---
id: fixture_elixir_visitor_iframe_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_iframe => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<h3>Reviews</h3><iframe src=\"https://widget.example.com/reviews\"></iframe><p>See reviews from our partners.</p>", visitor)

```
