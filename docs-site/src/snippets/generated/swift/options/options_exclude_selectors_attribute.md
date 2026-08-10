```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\"[role='complementary']\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options: _options)

```
