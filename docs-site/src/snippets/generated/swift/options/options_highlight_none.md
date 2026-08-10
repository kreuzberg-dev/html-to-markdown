```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"highlight_style\":\"None\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Text with <mark>plain</mark> content.</p>", options: _options)

```
