```elixir title="Elixir"
visitor = %{
      :handle_audio => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Background music:</p><audio src=\"music.ogg\" autoplay></audio><p>Enjoy!</p>", visitor)

```
