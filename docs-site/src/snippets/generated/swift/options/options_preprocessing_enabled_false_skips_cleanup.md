```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"enabled\":false}}")
_ = try HtmlToMarkdown.convert(html: "<nav>NavSection</nav><p>Paragraph</p>", options: _options)

```
