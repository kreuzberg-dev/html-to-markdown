```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<pre><code>some code</code></pre>', HtmlToMarkdownRs::ConversionOptions.new(code_block_style: 'Tildes'))

```
