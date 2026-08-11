---
id: fixture_elixir_script_tags_only
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>")

```
