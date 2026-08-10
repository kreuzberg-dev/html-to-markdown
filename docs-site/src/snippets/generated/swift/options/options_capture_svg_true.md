```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"capture_svg\":true,\"extract_images\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Below SVG:</p><svg xmlns=\"http://www.w3.org/2000/svg\" width=\"10\" height=\"10\"><rect width=\"10\" height=\"10\" fill=\"red\"/></svg>", options: _options)

```
