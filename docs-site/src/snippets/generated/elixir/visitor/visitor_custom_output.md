```elixir title="Elixir"
visitor = %{
      :handle_heading => fn(_args) ->
        {:custom, "\#\# REPLACED HEADING"}
      end,
    }

result = HtmlToMarkdown.convert("<h1>Original Heading</h1>", visitor)

```
