```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"preset" => "Minimal"}}
result = HtmlToMarkdown.convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", options_value)

```
