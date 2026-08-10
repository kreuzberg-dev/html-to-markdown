```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"escape_misc\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Use # and | and ~ in text.</p>", options: _options)

```
