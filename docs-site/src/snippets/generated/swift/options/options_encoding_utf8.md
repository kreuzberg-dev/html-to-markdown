```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"encoding\":\"utf-8\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Café naïve résumé</p>", options: _options)

```
