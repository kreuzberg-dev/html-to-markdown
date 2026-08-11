---
id: fixture_ruby_visitor_details_summary_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_details(ctx, is_open)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>', visitor)

```
