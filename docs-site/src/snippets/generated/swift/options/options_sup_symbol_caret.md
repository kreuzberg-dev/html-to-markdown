```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"sup_symbol\":\"^\"}")
_ = try HtmlToMarkdown.convert(html: "<p>x<sup>2</sup></p>", options: _options)

```
