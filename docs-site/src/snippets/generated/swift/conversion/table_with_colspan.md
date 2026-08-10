```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<table><thead><tr><th colspan=\"2\">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>", options: _options)

```
