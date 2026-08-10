```elixir title="Elixir"
visitor = %{
      :handle_element_end => fn(_args) ->
        {:custom, "MODIFIED OUTPUT"}
      end,
    }

result = HtmlToMarkdown.convert("<blockquote><p>Original quote</p></blockquote>", visitor)

```
