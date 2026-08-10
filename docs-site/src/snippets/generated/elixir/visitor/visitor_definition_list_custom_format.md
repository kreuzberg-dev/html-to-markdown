```elixir title="Elixir"
visitor = %{
      :handle_definition_description => fn(args) ->
        {:custom, "> " <> Map.get(args, "text", "")}
      end,
      :handle_definition_term => fn(args) ->
        {:custom, "\#\#\# " <> Map.get(args, "text", "")}
      end,
    }

result = HtmlToMarkdown.convert("<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", visitor)

```
