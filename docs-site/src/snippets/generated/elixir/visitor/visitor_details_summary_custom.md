```elixir title="Elixir"
visitor = %{
      :handle_summary => fn(args) ->
        {:custom, "[EXPANDABLE] " <> Map.get(args, "text", "")}
      end,
    }

result = HtmlToMarkdown.convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", visitor)

```
