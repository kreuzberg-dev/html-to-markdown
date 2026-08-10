```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"highlight_style\":\"DoubleEqual\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Text with <mark>highlighted</mark> here.</p>", options: _options)

```
