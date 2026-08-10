```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Item A<ol><li>Sub 1</li><li>Sub 2</li></ol></li><li>Item B</li></ul>", options: _options)

```
