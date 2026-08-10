```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"max_depth\":0}")
_ = try HtmlToMarkdown.convert(html: "<p>Hello</p>", options: _options)

```
