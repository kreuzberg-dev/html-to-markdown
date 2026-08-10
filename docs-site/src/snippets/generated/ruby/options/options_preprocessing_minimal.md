```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>Navigation</nav><p>Content</p><footer>Footer</footer>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'preset' => 'Minimal' }))

```
