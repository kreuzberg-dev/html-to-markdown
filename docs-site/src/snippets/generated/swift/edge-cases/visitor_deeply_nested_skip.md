```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorDeeplyNestedSkip: HtmlVisitorProtocol {
    func visitMark(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorDeeplyNestedSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<div><p>Outer <em>emphasis <strong>with bold <mark>and highlight</mark></strong></em> text</p></div>", options: _options)

```
