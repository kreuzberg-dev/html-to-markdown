```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<ol><li>First</li><li>Second</li><li>Third</li></ol>", options: _options)

```
