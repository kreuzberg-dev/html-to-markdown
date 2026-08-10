```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", options: _options)

```
