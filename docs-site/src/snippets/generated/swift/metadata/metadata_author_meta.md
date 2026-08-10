```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<html><head><title>Page</title><meta name=\"author\" content=\"Jane Doe\"></head><body><p>Content</p></body></html>", options: _options)

```
