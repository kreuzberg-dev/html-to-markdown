```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", options: _options)

```
