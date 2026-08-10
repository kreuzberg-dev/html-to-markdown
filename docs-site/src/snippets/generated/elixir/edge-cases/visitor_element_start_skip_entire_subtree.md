```elixir title="Elixir"
visitor = %{
      :handle_element_start => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<div><h1>Title</h1><p>Content</p></div>", visitor)

```
