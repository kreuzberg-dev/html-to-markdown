```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"default_title\":true}")
_ = try HtmlToMarkdown.convert(html: "<p><a href='https://example.com'>Link</a></p>", options: _options)

```
