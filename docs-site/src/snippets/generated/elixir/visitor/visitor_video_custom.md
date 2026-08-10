```elixir title="Elixir"
visitor = %{
      :handle_video => fn(args) ->
        {:custom, "[VIDEO: " <> Map.get(args, "src", "") <> "]"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Watch our tutorial:</p><video src=\"tutorial.mp4\" width=\"320\" height=\"240\" controls></video><p>Great content!</p>", visitor)

```
