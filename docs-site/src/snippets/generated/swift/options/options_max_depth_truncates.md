```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"max_depth\":3}")
_ = try HtmlToMarkdown.convert(html: "<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options: _options)

```
