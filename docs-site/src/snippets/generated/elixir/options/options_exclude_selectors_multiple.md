```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{exclude_selectors: [".nav", "footer"]}
result = HtmlToMarkdown.convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options_value)

```
