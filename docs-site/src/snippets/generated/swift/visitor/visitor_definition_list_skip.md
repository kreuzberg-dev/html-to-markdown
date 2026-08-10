```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorDefinitionListSkip: HtmlVisitorProtocol {
    func visitDefinitionTerm(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
    func visitDefinitionDescription(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorDefinitionListSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Glossary:</p><dl><dt>Term A</dt><dd>Definition of term A</dd><dt>Term B</dt><dd>Definition of term B</dd></dl><p>End of glossary</p>", options: _options)

```
