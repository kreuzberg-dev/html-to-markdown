```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<html><head><title>Test Page</title><meta name='description' content='A test page'></head><body><p>Content</p></body></html>", options: _options)

```
