```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"output_format\":\"Djot\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Simple paragraph.</p>", options: _options)

```
