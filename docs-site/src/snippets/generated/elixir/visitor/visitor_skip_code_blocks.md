---
id: fixture_elixir_visitor_skip_code_blocks
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_code_block => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", visitor)

```
