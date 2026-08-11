---
id: fixture_elixir_visitor_unknown_tag_preservation
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_custom_element => fn(_args) ->
        :preserve_html
      end,
    }

result = HtmlToMarkdown.convert("<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>", visitor)

```
