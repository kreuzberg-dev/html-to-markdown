```elixir title="Elixir"
visitor = %{
      :handle_image => fn(args) ->
        {:custom, "[Image: " <> Map.get(args, "alt", "") <> "]"}
      end,
    }

result = HtmlToMarkdown.convert("<img src=\"banner.png\" alt=\"Banner\">", visitor)

```
