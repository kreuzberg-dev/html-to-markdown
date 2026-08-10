```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"/docs/intro.html\">/docs/intro.html</a>", options: _options)

```
