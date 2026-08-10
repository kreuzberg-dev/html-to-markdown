```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"whitespace_mode\":\"Strict\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Preserved   spacing.</p>", options: _options)

```
