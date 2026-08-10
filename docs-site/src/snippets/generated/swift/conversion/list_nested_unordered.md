```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Parent A<ul><li>Child A1</li><li>Child A2</li></ul></li><li>Parent B</li></ul>", options: _options)

```
