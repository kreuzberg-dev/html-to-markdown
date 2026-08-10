```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSkipCodeBlocks: HtmlVisitorProtocol {
    func visitCodeBlock(_ ctx: HtmlToMarkdown.NodeContext, _ lang: String?, _ code: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSkipCodeBlocks())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", options: _options)

```
