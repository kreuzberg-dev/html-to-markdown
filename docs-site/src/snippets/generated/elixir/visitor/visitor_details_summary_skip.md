---
id: fixture_elixir_visitor_details_summary_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_details => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", visitor)

```
