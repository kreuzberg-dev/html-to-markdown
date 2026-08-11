---
id: fixture_elixir_visitor_audio_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_audio => fn(_args) ->
        {:custom, "[AUDIO: podcast.mp3]"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Listen to this: <audio src=\"podcast.mp3\" controls></audio></p>", visitor)

```
