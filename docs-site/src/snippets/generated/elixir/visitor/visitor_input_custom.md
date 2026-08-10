```elixir title="Elixir"
visitor = %{
      :handle_input => fn(args) ->
        {:custom, "[INPUT:" <> Map.get(args, "input_type", "") <> "]"}
      end,
    }

result = HtmlToMarkdown.convert("<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", visitor)

```
