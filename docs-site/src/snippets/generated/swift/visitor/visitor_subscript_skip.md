```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSubscriptSkip: HtmlVisitorProtocol {
    func visitSubscript(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSubscriptSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>The formula C<sub>12</sub>H<sub>22</sub>O<sub>11</sub> is sugar.</p>", options: _options)

```
