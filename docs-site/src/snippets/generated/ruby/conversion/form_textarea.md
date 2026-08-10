```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<form><label>Message:</label><textarea>Default text content</textarea></form>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'remove_forms' => false }))

```
