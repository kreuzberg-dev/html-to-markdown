---
id: fixture_elixir_structured_data_multiple_json_ld
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<html><head><title>Shop Page</title><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"Product\",\"name\":\"Widget\",\"price\":\"9.99\"}</script><script type=\"application/ld+json\">{\"@context\":\"https://schema.org\",\"@type\":\"BreadcrumbList\",\"itemListElement\":[{\"@type\":\"ListItem\",\"position\":1,\"name\":\"Home\"}]}</script></head><body><h1>Widget</h1><p>A great widget for all purposes.</p></body></html>")

```
