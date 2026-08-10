```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<nav>NavSection</nav><p>Paragraph</p>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'enabled' => false }))

```
