```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomLinkStatic: HtmlVisitorProtocol {
    func visitLink(_ ctx: HtmlToMarkdown.NodeContext, _ href: String, _ text: String, _ title: String?) -> VisitResult { return .custom(field0: "[REDACTED LINK]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomLinkStatic())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<a href=\"https://example.com\">Click here</a>", options: _options)

```
