```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorUnderlineCustom: HtmlVisitorProtocol {
    func visitUnderline(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "_\(text)_") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorUnderlineCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>This is <u>very important</u> text.</p>", options: _options)

```
