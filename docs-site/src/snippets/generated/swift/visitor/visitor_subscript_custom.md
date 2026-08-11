---
id: fixture_swift_visitor_subscript_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSubscriptCustom: HtmlVisitorProtocol {
    func visitSubscript(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "~\(text)~") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSubscriptCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>H<sub>2</sub>O is water.</p>", options: _options)

```
