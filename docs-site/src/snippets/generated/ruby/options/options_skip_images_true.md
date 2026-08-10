```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", HtmlToMarkdownRs::ConversionOptions.new(skip_images: true))

```
