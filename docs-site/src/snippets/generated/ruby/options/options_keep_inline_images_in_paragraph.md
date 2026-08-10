```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", HtmlToMarkdownRs::ConversionOptions.new(keep_inline_images_in: ['p']))

```
