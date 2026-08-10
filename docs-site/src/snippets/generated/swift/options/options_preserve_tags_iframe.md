```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preserve_tags\":[\"iframe\"]}")
_ = try HtmlToMarkdown.convert(html: "<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options: _options)

```
