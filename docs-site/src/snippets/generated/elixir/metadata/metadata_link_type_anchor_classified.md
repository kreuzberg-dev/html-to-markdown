---
id: fixture_elixir_metadata_link_type_anchor_classified
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p>Jump to <a href=\"\#section\">section</a> below.</p>", options_value)

```
