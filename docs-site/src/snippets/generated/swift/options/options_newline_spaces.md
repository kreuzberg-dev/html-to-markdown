```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"newline_style\":\"Spaces\"}")
_ = try HtmlToMarkdown.convert(html: "<p>First<br>Second</p>", options: _options)

```
