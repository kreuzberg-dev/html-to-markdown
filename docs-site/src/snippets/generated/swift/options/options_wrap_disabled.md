```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"wrap\":false}")
_ = try HtmlToMarkdown.convert(html: "<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options: _options)

```
