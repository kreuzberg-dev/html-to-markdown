```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"include_document_structure\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Example code:</p><pre><code class=\"language-rust\">fn main() { println!(\"Hello\"); }</code></pre>", options: _options)

```
