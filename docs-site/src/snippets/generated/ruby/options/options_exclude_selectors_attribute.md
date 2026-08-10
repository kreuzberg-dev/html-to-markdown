```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<body><div role="complementary">Sidebar</div><p>Primary text</p></body>', HtmlToMarkdownRs::ConversionOptions.new(exclude_selectors: ["[role='complementary']"]))

```
