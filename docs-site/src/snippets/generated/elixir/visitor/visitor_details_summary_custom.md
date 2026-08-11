---
id: fixture_elixir_visitor_details_summary_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_summary => fn(args) ->
        {:custom, "[EXPANDABLE] " <> Map.get(args, "text", "")}
      end,
    }

result = HtmlToMarkdown.convert("<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", visitor)

```
