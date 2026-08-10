```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>', HtmlToMarkdownRs::ConversionOptions.new(compact_tables: false))

```
