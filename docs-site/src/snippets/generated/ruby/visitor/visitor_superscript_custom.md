```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_superscript(ctx, text)
    { Custom: "^#{text}^" }
  end
end.new
result = HtmlToMarkdown.convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", visitor)

```
