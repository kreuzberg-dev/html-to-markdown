```ruby title="Ruby"
require "html-to-markdown"
visitor = Class.new do
  def visit_custom_element(ctx, tag_name, html)
    'PreserveHtml'
  end
end.new
result = HtmlToMarkdown.convert('<article><p>Article text</p><x-custom>Custom element with content</x-custom><p>More article text</p></article>', visitor)

```
