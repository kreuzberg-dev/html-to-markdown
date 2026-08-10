```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorPreserveHtml: HtmlVisitorProtocol {
    func visitCustomElement(_ ctx: HtmlToMarkdown.NodeContext, _ tagName: String, _ html: String) -> VisitResult { return .preserveHtml }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorPreserveHtml())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<div><custom-tag>Custom content</custom-tag></div>", options: _options)

```
