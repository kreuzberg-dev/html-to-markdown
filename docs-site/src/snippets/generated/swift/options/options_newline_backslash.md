```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"newline_style\":\"Backslash\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Line one<br>Line two</p>", options: _options)

```
