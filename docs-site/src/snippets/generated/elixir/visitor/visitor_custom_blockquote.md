```elixir title="Elixir"
visitor = %{
      :handle_blockquote => fn(args) ->
        {:custom, "QUOTE: \"" <> Map.get(args, "content", "") <> "\""}
      end,
    }

result = HtmlToMarkdown.convert("<blockquote><p>A wise quote.</p></blockquote>", visitor)

```
