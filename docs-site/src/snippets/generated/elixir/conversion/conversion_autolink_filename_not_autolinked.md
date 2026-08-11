---
id: fixture_elixir_conversion_autolink_filename_not_autolinked
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<a href=\"foobar.png\">foobar.png</a>")

```
