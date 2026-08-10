```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"output_format\":\"Plain\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options: _options)

```
