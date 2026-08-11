---
id: fixture_elixir_visitor_figure_custom
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_figcaption => fn(args) ->
        {:custom, "*" <> Map.get(args, "text", "") <> "*"}
      end,
    }

result = HtmlToMarkdown.convert("<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src=\"diagram.png\" alt=\"System architecture diagram\"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>", visitor)

```
