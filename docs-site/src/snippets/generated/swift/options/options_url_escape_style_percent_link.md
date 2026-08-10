```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"url_escape_style\":\"percent\"}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"/file (1).pdf\">file</a>", options: _options)

```
