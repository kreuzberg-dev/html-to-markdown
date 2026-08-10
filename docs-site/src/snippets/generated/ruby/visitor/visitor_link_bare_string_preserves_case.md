```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_link(ctx, href, text, title)
    "[#{text}](https://new-cdn.com/file.pdf)"
  end
end.new
result = HtmlToMarkdown.convert('<a href="https://old-cdn.com/file.pdf">Download</a>', visitor)

```
