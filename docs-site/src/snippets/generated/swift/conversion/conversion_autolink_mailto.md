```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"mailto:a@b.com\">a@b.com</a>", options: _options)

```
