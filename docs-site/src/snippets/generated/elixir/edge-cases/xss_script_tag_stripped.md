---
id: fixture_elixir_xss_script_tag_stripped
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>")

```
