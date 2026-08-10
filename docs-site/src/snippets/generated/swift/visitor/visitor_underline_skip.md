```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorUnderlineSkip: HtmlVisitorProtocol {
    func visitUnderline(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorUnderlineSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Normal text with <u>underlined part</u> and more text.</p>", options: _options)

```
