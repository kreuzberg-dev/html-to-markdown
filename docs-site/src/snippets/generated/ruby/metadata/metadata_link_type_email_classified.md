---
id: fixture_ruby_metadata_link_type_email_classified
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Contact <a href="mailto:hello@example.com">us</a> directly.</p>', HtmlToMarkdownRs::ConversionOptions.new(extract_metadata: true))

```
