```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_definition_description(ctx, text)
    { Custom: "> #{text}" }
  end
  def visit_definition_term(ctx, text)
    { Custom: "### #{text}" }
  end
end.new
result = HtmlToMarkdown.convert('<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>', visitor)

```
