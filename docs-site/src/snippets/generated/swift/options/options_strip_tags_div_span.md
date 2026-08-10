```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"strip_tags\":[\"div\",\"span\"]}")
_ = try HtmlToMarkdown.convert(html: "<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", options: _options)

```
