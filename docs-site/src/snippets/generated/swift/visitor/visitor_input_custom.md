```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorInputCustom: HtmlVisitorProtocol {
    func visitInput(_ ctx: HtmlToMarkdown.NodeContext, _ inputType: String, _ name: String?, _ value: String?) -> VisitResult { return .custom(field0: "[INPUT:\(inputType)]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorInputCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<form><label>Username: <input type=\"text\" name=\"username\" value=\"\"></label><label>Password: <input type=\"password\" name=\"password\"></label></form>", options: _options)

```
