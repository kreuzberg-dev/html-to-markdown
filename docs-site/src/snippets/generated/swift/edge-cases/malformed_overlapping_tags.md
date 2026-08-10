```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p><b><i>bold and italic</b></i></p>", options: _options)

```
