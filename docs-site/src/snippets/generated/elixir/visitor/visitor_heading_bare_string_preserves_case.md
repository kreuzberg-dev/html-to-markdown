```elixir title="Elixir"
visitor = %{
      :handle_heading => fn(args) ->
        {:custom, "\#\# " <> Map.get(args, "text", "") <> " \#\#"}
      end,
    }

result = HtmlToMarkdown.convert("<h2>Important Section Title</h2><p>Body.</p>", visitor)

```
