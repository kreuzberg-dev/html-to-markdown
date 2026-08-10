```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<table><thead><tr><th>Expression</th><th>Result</th></tr></thead><tbody><tr><td>a | b</td><td>true</td></tr></tbody></table>", options: _options)

```
