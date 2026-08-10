```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorIframeCustom: HtmlVisitorProtocol {
    func visitIframe(_ ctx: HtmlToMarkdown.NodeContext, _ src: String?) -> VisitResult { return .custom(field0: "[EMBEDDED: https://maps.example.com/embed]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorIframeCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", options: _options)

```
