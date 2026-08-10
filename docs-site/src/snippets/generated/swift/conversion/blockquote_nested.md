```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>", options: _options)

```
