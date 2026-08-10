```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", options: _options)

```
