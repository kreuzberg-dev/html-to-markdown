```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"autolinks\":false}")
_ = try HtmlToMarkdown.convert(html: "<p><a href='https://example.com'>https://example.com</a></p>", options: _options)

```
