```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_images\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options: _options)

```
