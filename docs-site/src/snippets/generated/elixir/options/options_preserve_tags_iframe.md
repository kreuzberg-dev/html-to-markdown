---
id: fixture_elixir_options_preserve_tags_iframe
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preserve_tags: ["iframe"]}
result = HtmlToMarkdown.convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options_value)

```
