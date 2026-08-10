```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSkipLinks: HtmlVisitorProtocol {
    func visitLink(_ ctx: HtmlToMarkdown.NodeContext, _ href: String, _ text: String, _ title: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSkipLinks())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Before <a href=\"https://example.com\">link text</a> after</p>", options: _options)

```
