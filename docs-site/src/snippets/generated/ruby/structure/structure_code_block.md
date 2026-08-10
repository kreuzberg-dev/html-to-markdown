```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert('<p>Example code:</p><pre><code class="language-rust">fn main() { println!("Hello"); }</code></pre>', HtmlToMarkdownRs::ConversionOptions.new(include_document_structure: true))

```
