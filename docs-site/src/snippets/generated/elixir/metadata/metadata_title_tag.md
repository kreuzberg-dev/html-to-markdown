---
id: fixture_elixir_metadata_title_tag
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<html><head><title>My Page</title></head><body><p>Content</p></body></html>", options_value)

```
