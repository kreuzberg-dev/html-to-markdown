```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div class="cookie-banner">Accept cookies</div><p>Main content</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.cookie-banner']))

```
