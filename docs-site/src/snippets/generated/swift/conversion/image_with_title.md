```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", options: _options)

```
