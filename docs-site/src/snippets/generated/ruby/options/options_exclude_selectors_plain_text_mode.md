```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div class="nav">Navigation</div><p>Article body</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.nav'], output_format: 'Plain'))

```
