---
id: fixture_ruby_options_preprocessing_remove_navigation_false_keeps_nav
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'remove_navigation' => false }))

```
