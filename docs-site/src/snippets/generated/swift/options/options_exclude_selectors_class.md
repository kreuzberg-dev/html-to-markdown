```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".cookie-banner\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options: _options)

```
