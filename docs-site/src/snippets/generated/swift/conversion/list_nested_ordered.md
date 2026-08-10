```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>", options: _options)

```
