```elixir title="Elixir"
visitor = %{
      :handle_link => fn(_args) ->
        {:custom, "[REDACTED LINK]"}
      end,
    }

result = HtmlToMarkdown.convert("<a href=\"https://example.com\">Click here</a>", visitor)

```
