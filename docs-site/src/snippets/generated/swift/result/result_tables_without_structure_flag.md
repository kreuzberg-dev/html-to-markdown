```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", options: _options)

```
