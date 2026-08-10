```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_language\":\"python\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code>def hello(): pass</code></pre>", options: _options)

```
