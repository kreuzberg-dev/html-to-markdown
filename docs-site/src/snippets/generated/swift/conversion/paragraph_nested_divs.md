```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<div><div><p>Nested text</p></div></div>", options: _options)

```
