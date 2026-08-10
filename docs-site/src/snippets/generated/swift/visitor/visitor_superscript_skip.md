```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSuperscriptSkip: HtmlVisitorProtocol {
    func visitSuperscript(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSuperscriptSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>The equation x<sup>3</sup> + y<sup>3</sup> = z<sup>3</sup> has no solutions.</p>", options: _options)

```
