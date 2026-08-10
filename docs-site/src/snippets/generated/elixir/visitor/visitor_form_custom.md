```elixir title="Elixir"
visitor = %{
      :handle_form => fn(_args) ->
        {:custom, "[FORM PLACEHOLDER]"}
      end,
    }

result = HtmlToMarkdown.convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", visitor)

```
