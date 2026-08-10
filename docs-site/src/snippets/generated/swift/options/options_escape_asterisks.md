```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"escape_asterisks\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Use 2*3 = 6 in math.</p>", options: _options)

```
