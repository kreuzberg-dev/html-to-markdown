```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", options: _options)

```
