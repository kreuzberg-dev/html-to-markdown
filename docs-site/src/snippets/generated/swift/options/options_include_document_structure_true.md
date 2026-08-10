```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"include_document_structure\":true}")
_ = try HtmlToMarkdown.convert(html: "<article><h1>Heading</h1><p>Paragraph body.</p></article>", options: _options)

```
