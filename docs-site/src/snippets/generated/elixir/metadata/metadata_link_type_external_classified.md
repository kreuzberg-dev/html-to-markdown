---
id: fixture_elixir_metadata_link_type_external_classified
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", options_value)

```
