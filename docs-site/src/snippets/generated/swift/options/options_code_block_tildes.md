```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Tildes\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code>let x = 1;</code></pre>", options: _options)

```
