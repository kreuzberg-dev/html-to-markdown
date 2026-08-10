```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_image(ctx, src, alt, title)
    { Custom: "[Image: #{alt}]" }
  end
end.new
result = HtmlToMarkdown.convert('<img src="banner.png" alt="Banner">', visitor)

```
