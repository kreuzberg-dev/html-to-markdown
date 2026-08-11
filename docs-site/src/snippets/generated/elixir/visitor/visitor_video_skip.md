---
id: fixture_elixir_visitor_video_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_video => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", visitor)

```
