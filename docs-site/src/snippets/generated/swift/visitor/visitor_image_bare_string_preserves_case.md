```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorImageBareStringPreservesCase: HtmlVisitorProtocol {
    func visitImage(_ ctx: HtmlToMarkdown.NodeContext, _ src: String, _ alt: String, _ title: String?) -> VisitResult { return .custom(field0: "[image: \(alt) -> \(src)]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorImageBareStringPreservesCase())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", options: _options)

```
