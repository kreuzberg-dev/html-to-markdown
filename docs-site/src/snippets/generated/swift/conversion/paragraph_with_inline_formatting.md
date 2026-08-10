```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href=\"https://example.com\">link</a>.</p>", options: _options)

```
