```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorFormCustom: HtmlVisitorProtocol {
    func visitForm(_ ctx: HtmlToMarkdown.NodeContext, _ action: String?, _ method: String?) -> VisitResult { return .custom(field0: "[FORM PLACEHOLDER]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorFormCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", options: _options)

```
