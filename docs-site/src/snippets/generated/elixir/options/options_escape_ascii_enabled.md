---
id: fixture_elixir_options_escape_ascii_enabled
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{escape_ascii: true}
result = HtmlToMarkdown.convert("<p>Text with \# hash and [brackets] and * star</p>", options_value)

```
