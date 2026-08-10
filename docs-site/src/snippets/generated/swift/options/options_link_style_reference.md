```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"link_style\":\"Reference\"}")
_ = try HtmlToMarkdown.convert(html: "<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options: _options)

```
