```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_definition_term(ctx, text)
    { Custom: "**#{text}**" }
  end
end.new
result = HtmlToMarkdown.convert('<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>', visitor)

```
