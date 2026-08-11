---
id: fixture_swift_visitor_superscript_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSuperscriptCustom: HtmlVisitorProtocol {
    func visitSuperscript(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "^\(text)^") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSuperscriptCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", options: _options)

```
