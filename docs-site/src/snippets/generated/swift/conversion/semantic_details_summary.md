```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", options: _options)

```
