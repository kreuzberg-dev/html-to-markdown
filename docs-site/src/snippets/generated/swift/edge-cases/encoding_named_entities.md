```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Em dash&mdash;used for parenthetical remarks&mdash;is common. Ellipsis&hellip; indicates omission. Non-breaking&nbsp;space.</p>", options: _options)

```
