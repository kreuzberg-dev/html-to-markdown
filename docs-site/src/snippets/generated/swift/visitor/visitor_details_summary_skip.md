```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorDetailsSummarySkip: HtmlVisitorProtocol {
    func visitDetails(_ ctx: HtmlToMarkdown.NodeContext, _ open: Bool) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorDetailsSummarySkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Main content here.</p><details><summary>Hidden section</summary><p>Secret details</p></details><p>More main content.</p>", options: _options)

```
