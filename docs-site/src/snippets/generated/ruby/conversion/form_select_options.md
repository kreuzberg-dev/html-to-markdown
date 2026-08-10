```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<form><label>Color:</label><select><option value="red">Red</option><option value="blue" selected>Blue</option><option value="green">Green</option></select></form>', HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'remove_forms' => false }))

```
