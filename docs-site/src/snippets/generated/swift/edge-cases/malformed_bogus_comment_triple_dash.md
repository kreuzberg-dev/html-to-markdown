```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", options: _options)

```
