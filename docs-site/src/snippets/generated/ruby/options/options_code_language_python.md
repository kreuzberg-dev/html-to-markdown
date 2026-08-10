```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<pre><code>def hello(): pass</code></pre>', HtmlToMarkdownRs::ConversionOptions.new(code_language: 'python'))

```
