```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div id="ad-container">Buy stuff</div><p>Article text</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['#ad-container']))

```
