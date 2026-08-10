```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomLinkFormat: HtmlVisitorProtocol {
    func visitLink(_ ctx: HtmlToMarkdown.NodeContext, _ href: String, _ text: String, _ title: String?) -> VisitResult { return .custom(field0: "\(text) (\(href))") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomLinkFormat())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Visit <a href=\"https://example.com\">Example</a> for more info.</p>", options: _options)

```
