```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorIframeSkip: HtmlVisitorProtocol {
    func visitIframe(_ ctx: HtmlToMarkdown.NodeContext, _ src: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorIframeSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<h3>Reviews</h3><iframe src=\"https://widget.example.com/reviews\"></iframe><p>See reviews from our partners.</p>", options: _options)

```
