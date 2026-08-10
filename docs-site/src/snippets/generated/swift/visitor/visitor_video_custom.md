```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorVideoCustom: HtmlVisitorProtocol {
    func visitVideo(_ ctx: HtmlToMarkdown.NodeContext, _ src: String?) -> VisitResult { return .custom(field0: "[VIDEO: \(src ?? "")]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorVideoCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Watch our tutorial:</p><video src=\"tutorial.mp4\" width=\"320\" height=\"240\" controls></video><p>Great content!</p>", options: _options)

```
