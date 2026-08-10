```elixir title="Elixir"
visitor = %{
      :handle_custom_element => fn(_args) ->
        {:custom, "[CUSTOM WIDGET]"}
      end,
    }

result = HtmlToMarkdown.convert("<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", visitor)

```
