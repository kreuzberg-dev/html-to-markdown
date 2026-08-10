```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorFigureCustom: HtmlVisitorProtocol {
    func visitFigcaption(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "*\(text)*") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorFigureCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<article><h1>Article Title</h1><p>Introduction paragraph.</p><figure><img src=\"diagram.png\" alt=\"System architecture diagram\"><figcaption>Figure 1: System Architecture</figcaption></figure><p>Explanation of the figure.</p></article>", options: _options)

```
