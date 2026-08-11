---
id: fixture_elixir_visitor_figure_custom_wrap
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_figure_end => fn(args) ->
        {:custom, Map.get(args, "output", "") <> "\n[/FIGURE]\n"}
      end,
      :handle_figure_start => fn(_args) ->
        {:custom, "\n[FIGURE]\n"}
      end,
    }

result = HtmlToMarkdown.convert("<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", visitor)

```
