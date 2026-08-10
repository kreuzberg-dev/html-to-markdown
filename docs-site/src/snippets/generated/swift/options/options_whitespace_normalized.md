```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"whitespace_mode\":\"Normalized\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Text   with    extra   spaces.</p>", options: _options)

```
