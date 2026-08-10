```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"preset" => "Aggressive"}}
result = HtmlToMarkdown.convert("<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>", options_value)

```
