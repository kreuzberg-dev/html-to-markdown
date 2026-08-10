```elixir title="Elixir"
visitor = %{
      :handle_definition_description => fn(_args) ->
        :skip
      end,
      :handle_definition_term => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", visitor)

```
