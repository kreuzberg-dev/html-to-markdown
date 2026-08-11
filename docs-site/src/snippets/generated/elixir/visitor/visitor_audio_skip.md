---
id: fixture_elixir_visitor_audio_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_audio => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", visitor)

```
