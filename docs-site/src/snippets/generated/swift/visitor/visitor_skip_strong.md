```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSkipStrong: HtmlVisitorProtocol {
    func visitStrong(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSkipStrong())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Normal <strong>bold text</strong> normal</p>", options: _options)

```
