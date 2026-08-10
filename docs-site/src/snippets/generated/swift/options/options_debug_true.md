```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"debug\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Debug test</p>", options: _options)

```
