---
id: fixture_elixir_visitor_definition_list_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_definition_description => fn(_args) ->
        :skip
      end,
      :handle_definition_term => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", visitor)

```
