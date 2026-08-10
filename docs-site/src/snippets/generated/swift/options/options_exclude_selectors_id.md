```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\"#ad-container\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options: _options)

```
