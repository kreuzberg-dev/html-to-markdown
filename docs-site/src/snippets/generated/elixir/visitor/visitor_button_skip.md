```elixir title="Elixir"
visitor = %{
      :handle_button => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Actions available: <button>Save</button> <button>Delete</button> <button>Export</button></p>", visitor)

```
