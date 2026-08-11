---
id: fixture_elixir_metadata_link_type_email_classified
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{extract_metadata: true}
result = HtmlToMarkdown.convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options_value)

```
