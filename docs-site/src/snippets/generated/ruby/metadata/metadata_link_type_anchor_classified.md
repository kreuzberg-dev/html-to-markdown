```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Jump to <a href="#section">section</a> below.</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
