```elixir title="Elixir"
visitor = %{
      :handle_custom_element => fn(_args) ->
        :preserve_html
      end,
    }

result = HtmlToMarkdown.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", visitor)

```
