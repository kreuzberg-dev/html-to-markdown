```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_blockquote(ctx, content, depth)
    { Custom: "QUOTE: \"#{content}\"" }
  end
end.new
result = HtmlToMarkdown.convert('<blockquote><p>A wise quote.</p></blockquote>', visitor)

```
