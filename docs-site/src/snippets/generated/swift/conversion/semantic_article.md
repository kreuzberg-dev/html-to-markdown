```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<article><h2>Article Title</h2><p>Article body.</p></article>", options: _options)

```
