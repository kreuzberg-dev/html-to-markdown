---
id: fixture_elixir_form_textarea
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_forms" => false}}
result = HtmlToMarkdown.convert("<form><label>Message:</label><textarea>Default text content</textarea></form>", options_value)

```
