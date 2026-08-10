```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomEmphasis: HtmlVisitorProtocol {
    func visitEmphasis(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: ">>>\(text)<<<") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomEmphasis())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>This is <em>important</em> text.</p>", options: _options)

```
