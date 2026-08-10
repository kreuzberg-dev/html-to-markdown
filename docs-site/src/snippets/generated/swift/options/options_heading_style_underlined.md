```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"heading_style\":\"Underlined\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Main Title</h1>", options: _options)

```
