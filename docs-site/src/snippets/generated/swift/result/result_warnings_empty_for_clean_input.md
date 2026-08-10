```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", options: _options)

```
