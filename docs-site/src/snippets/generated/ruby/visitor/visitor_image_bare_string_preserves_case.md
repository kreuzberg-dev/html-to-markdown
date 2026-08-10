```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_image(ctx, src, alt, title)
    "[image: #{alt} -> #{src}]"
  end
end.new
result = HtmlToMarkdown.convert('<img src="PhotoOne.JPG" alt="Sunset Over Bay">', visitor)

```
