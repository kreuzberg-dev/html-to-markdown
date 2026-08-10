```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Indented\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code>print('hello')</code></pre>", options: _options)

```
