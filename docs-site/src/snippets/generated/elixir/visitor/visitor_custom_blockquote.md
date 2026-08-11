---
id: fixture_elixir_visitor_custom_blockquote
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_blockquote => fn(args) ->
        {:custom, "QUOTE: \"" <> Map.get(args, "content", "") <> "\""}
      end,
    }

result = HtmlToMarkdown.convert("<blockquote><p>A wise quote.</p></blockquote>", visitor)

```
