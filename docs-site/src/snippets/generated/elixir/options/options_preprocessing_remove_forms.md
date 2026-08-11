---
id: fixture_elixir_options_preprocessing_remove_forms
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_forms" => true}}
result = HtmlToMarkdown.convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options_value)

```
