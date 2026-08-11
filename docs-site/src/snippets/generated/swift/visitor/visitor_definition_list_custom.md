---
id: fixture_swift_visitor_definition_list_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorDefinitionListCustom: HtmlVisitorProtocol {
    func visitDefinitionTerm(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "**\(text)**") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorDefinitionListCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>", options: _options)

```
