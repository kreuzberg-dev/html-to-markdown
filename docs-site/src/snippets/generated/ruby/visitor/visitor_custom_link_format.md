```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_link(ctx, href, text, title)
    { Custom: "#{text} (#{href})" }
  end
end.new
result = HtmlToMarkdown.convert('<p>Visit <a href="https://example.com">Example</a> for more info.</p>', visitor)

```
