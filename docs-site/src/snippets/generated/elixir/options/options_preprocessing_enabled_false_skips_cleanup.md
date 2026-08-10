```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"enabled" => false}}
result = HtmlToMarkdown.convert("<nav>NavSection</nav><p>Paragraph</p>", options_value)

```
