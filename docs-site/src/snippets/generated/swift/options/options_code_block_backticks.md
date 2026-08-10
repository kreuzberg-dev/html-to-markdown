```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Backticks\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code class=\"language-js\">console.log('hi');</code></pre>", options: _options)

```
