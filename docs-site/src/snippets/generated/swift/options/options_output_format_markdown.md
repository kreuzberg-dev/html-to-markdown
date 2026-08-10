```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"heading_style\":\"Atx\",\"output_format\":\"Markdown\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><p>Some text.</p>", options: _options)

```
