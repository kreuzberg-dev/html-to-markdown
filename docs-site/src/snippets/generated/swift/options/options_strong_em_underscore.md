```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"strong_em_symbol\":\"_\"}")
_ = try HtmlToMarkdown.convert(html: "<p><strong>bold</strong> and <em>italic</em></p>", options: _options)

```
