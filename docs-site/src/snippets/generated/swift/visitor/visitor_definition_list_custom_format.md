```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorDefinitionListCustomFormat: HtmlVisitorProtocol {
    func visitDefinitionTerm(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "### \(text)") }
    func visitDefinitionDescription(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "> \(text)") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorDefinitionListCustomFormat())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", options: _options)

```
