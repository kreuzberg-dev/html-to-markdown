```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", options: _options)

```
