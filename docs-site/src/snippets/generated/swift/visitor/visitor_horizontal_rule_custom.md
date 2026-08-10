```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorHorizontalRuleCustom: HtmlVisitorProtocol {
    func visitHorizontalRule(_ ctx: HtmlToMarkdown.NodeContext) -> VisitResult { return .custom(field0: "\n[DIVIDER]\n") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorHorizontalRuleCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", options: _options)

```
