---
id: fixture_elixir_visitor_figure_skip
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
visitor = %{
      :handle_figure_start => fn(_args) ->
        :skip
      end,
    }

result = HtmlToMarkdown.convert("<p>See the chart below:</p><figure><img src=\"chart.svg\"><figcaption>Revenue Trends 2020-2024</figcaption></figure><p>As shown in the chart above.</p>", visitor)

```
