```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<pre><code class=\"language-js\">console.log('hi');</code></pre>", HtmlToMarkdownRs::ConversionOptions.new(code_block_style: 'Backticks'))

```
