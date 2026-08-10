```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorInputSkip: HtmlVisitorProtocol {
    func visitInput(_ ctx: HtmlToMarkdown.NodeContext, _ inputType: String, _ name: String?, _ value: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorInputSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Sign up:</p><input type=\"text\" name=\"email\" placeholder=\"your@email.com\"><input type=\"checkbox\" name=\"agree\"><p>Continue</p>", options: _options)

```
