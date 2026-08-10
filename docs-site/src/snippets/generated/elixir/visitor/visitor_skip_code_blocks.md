```elixir title="Elixir"
visitor = %{
      :handle_code_block => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", visitor)

```
