```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorHeadingBareStringPreservesCase: HtmlVisitorProtocol {
    func visitHeading(_ ctx: HtmlToMarkdown.NodeContext, _ level: UInt32, _ text: String, _ id: String?) -> VisitResult { return .custom(field0: "## \(text) ##") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorHeadingBareStringPreservesCase())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<h2>Important Section Title</h2><p>Body.</p>", options: _options)

```
