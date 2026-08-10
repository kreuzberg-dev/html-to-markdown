```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<div><h1>Title<p>First paragraph<p>Second paragraph</div>", options: _options)

```
