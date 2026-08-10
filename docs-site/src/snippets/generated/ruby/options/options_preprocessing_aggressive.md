```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'preset' => 'Aggressive' }))

```
