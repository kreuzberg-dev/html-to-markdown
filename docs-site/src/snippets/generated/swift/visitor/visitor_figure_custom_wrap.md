```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorFigureCustomWrap: HtmlVisitorProtocol {
    func visitFigureStart(_ ctx: HtmlToMarkdown.NodeContext) -> VisitResult { return .custom(field0: "\n[FIGURE]\n") }
    func visitFigureEnd(_ ctx: HtmlToMarkdown.NodeContext, _ output: String) -> VisitResult { return .custom(field0: "\(output)\n[/FIGURE]\n") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorFigureCustomWrap())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", options: _options)

```
