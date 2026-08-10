```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"list_indent_width\":4}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", options: _options)

```
