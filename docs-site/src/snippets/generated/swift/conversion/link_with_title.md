```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"https://example.com\" title=\"Example Site\">Example</a>", options: _options)

```
