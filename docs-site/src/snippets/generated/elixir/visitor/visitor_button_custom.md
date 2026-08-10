```elixir title="Elixir"
visitor = %{
      :handle_button => fn(args) ->
        {:custom, "[BTN:" <> Map.get(args, "text", "") <> "]"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", visitor)

```
