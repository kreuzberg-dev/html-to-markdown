```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomOutput: HtmlVisitorProtocol {
    func visitHeading(_ ctx: HtmlToMarkdown.NodeContext, _ level: UInt32, _ text: String, _ id: String?) -> VisitResult { return .custom(field0: "## REPLACED HEADING") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomOutput())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<h1>Original Heading</h1>", options: _options)

```
