---
id: fixture_ruby_visitor_video_skip
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_video(ctx, src)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<h2>Demo</h2><video src="demo.webm"></video><p>See the demo above.</p>', visitor)

```
