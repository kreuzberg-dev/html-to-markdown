```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"br_in_tables\":false}")
_ = try HtmlToMarkdown.convert(html: "<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options: _options)

```
