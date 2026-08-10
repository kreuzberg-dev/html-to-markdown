```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><nav class="nav">Menu</nav><p>Content</p><footer>Footer</footer></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ['.nav', 'footer']))

```
