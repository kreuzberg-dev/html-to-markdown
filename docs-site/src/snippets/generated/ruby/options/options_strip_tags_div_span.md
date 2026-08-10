```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", HtmlToMarkdownRs::ConversionOptions.new(strip_tags: ['div', 'span']))

```
