---
id: fixture_elixir_options_max_depth_default_unlimited
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = HtmlToMarkdown.convert("<div><div><div><div><p>Deep content</p></div></div></div></div>")

```
