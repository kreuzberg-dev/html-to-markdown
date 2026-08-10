```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>", options: _options)

```
