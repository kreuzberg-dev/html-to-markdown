---
id: fixture_elixir_options_preprocessing_remove_navigation_false_keeps_nav
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
options_value = %HtmlToMarkdown.ConversionOptions{preprocessing: %{"remove_navigation" => false}}
result = HtmlToMarkdown.convert("<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>", options_value)

```
