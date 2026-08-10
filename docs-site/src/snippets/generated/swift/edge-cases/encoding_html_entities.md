```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", options: _options)

```
