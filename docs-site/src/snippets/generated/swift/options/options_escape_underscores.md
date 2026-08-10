```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"escape_underscores\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>The variable_name is defined.</p>", options: _options)

```
