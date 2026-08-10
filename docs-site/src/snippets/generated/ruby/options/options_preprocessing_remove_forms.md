```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", HtmlToMarkdownRs::ConversionOptions.new(preprocessing: { 'remove_forms' => true }))

```
