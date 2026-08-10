```elixir title="Elixir"
visitor = %{
      :handle_custom_element => fn(_args) ->
        :preserve_html
      end,
    }

result = HtmlToMarkdown.convert("<div><custom-tag>Custom content</custom-tag></div>", visitor)

```
