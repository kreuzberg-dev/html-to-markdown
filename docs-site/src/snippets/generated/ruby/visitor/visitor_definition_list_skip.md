```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_definition_description(ctx, text)
    'Skip'
  end
  def visit_definition_term(ctx, text)
    'Skip'
  end
end.new
result = HtmlToMarkdown.convert('<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>', visitor)

```
