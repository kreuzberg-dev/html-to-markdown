```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<div><div><div><div><p>Deep content</p></div></div></div></div>", options: _options)

```
