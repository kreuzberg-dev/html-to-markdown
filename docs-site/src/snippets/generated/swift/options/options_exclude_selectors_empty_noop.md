```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[]}")
_ = try HtmlToMarkdown.convert(html: "<p>Hello world</p>", options: _options)

```
