---
id: fixture_elixir_result_warnings_empty_for_clean_input
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>")

```
