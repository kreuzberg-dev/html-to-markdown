```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomImage: HtmlVisitorProtocol {
    func visitImage(_ ctx: HtmlToMarkdown.NodeContext, _ src: String, _ alt: String, _ title: String?) -> VisitResult { return .custom(field0: "[Image: \(alt)]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomImage())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<img src=\"banner.png\" alt=\"Banner\">", options: _options)

```
