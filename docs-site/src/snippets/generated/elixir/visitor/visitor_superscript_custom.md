```elixir title="Elixir"
visitor = %{
      :handle_superscript => fn(args) ->
        {:custom, "^" <> Map.get(args, "text", "") <> "^"}
      end,
    }

result = HtmlToMarkdown.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", visitor)

```
