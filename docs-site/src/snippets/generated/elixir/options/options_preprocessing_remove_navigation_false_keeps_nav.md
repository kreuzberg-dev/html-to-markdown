```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_navigation" => false}}
result = HtmlToMarkdown.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options_value)

```
