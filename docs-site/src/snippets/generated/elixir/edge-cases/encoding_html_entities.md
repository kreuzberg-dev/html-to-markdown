---
id: fixture_elixir_encoding_html_entities
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>")

```
