```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"br_in_tables\":true}")
_ = try HtmlToMarkdown.convert(html: "<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options: _options)

```
