```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".wrapper\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options: _options)

```
