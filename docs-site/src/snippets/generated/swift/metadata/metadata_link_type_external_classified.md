```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>See <a href=\"https://example.com\">Example</a> for details.</p>", options: _options)

```
