```elixir title="Elixir"
visitor = %{
      :handle_link => fn(args) ->
        {:custom, Map.get(args, "text", "") <> " (" <> Map.get(args, "href", "") <> ")"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", visitor)

```
