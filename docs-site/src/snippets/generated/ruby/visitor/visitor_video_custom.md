---
id: fixture_ruby_visitor_video_custom
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
    { Custom: "[VIDEO: #{src}]" }
  end
end.new
result = HtmlToMarkdown.convert('<p>Watch our tutorial:</p><video src="tutorial.mp4" width="320" height="240" controls></video><p>Great content!</p>', visitor)

```
