```elixir title="Elixir"
visitor = %{
      :handle_line_break => fn(_args) ->
        {:custom, " | "}
      end,
    }

result = HtmlToMarkdown.convert("<p>First line<br>Second line<br>Third line</p>", visitor)

```
