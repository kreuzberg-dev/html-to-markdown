---
id: fixture_elixir_visitor_skip_images
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_image => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", visitor)

```
