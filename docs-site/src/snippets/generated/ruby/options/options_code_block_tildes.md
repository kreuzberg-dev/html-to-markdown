```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<pre><code>let x = 1;</code></pre>', HtmlToMarkdownRs::ConversionOptions.new(code_block_style: 'Tildes'))

```
