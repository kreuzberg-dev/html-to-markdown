---
id: fixture_elixir_options_exclude_selectors_attribute
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: ["[role='complementary']"]}
result = HtmlToMarkdown.convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options_value)

```
