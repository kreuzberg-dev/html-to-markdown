```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'remove_navigation' => false }))

```
