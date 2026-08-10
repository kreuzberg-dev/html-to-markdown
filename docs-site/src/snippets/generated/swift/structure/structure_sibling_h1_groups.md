```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"include_document_structure\":true}")
_ = try HtmlToMarkdown.convert(html: "<h1>Chapter One</h1><h2>Section A</h2><p>Section A content.</p><h1>Chapter Two</h1><h2>Section B</h2><p>Section B content.</p>", options: _options)

```
