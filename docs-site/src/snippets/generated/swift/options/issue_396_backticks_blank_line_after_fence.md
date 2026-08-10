```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Backticks\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options: _options)

```
