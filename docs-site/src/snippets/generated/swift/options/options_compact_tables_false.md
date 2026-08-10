```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"compact_tables\":false}")
_ = try HtmlToMarkdown.convert(html: "<table><thead><tr><th>Name</th><th>Score</th></tr></thead><tbody><tr><td>Alice</td><td>100</td></tr><tr><td>Bob</td><td>42</td></tr></tbody></table>", options: _options)

```
