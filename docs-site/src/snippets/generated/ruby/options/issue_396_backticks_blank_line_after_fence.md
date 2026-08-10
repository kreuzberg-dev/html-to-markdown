```ruby title="Ruby"
require "html-to-markdown"
result = HtmlToMarkdown.convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", HtmlToMarkdownRs::ConversionOptions.new(code_block_style: 'Backticks'))

```
