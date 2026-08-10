```elixir title="Elixir"
visitor = %{
      :handle_definition_term => fn(args) ->
        {:custom, "**" <> Map.get(args, "text", "") <> "**"}
      end,
    }

result = HtmlToMarkdown.convert("<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", visitor)

```
