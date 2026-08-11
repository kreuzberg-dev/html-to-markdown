---
id: fixture_elixir_conversion_autolink_relative_path_not_autolinked
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>")

```
