```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorMarkSkip: HtmlVisitorProtocol {
    func visitMark(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorMarkSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Key insight: <mark>always validate input</mark> for security.</p>", options: _options)

```
