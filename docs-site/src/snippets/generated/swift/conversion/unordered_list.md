```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", options: _options)

```
