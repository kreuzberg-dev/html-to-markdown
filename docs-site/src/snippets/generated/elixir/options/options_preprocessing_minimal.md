---
id: fixture_elixir_options_preprocessing_minimal
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"preset" => "Minimal"}}
result = HtmlToMarkdown.convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", options_value)

```
