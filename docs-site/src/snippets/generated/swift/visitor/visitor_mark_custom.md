---
id: fixture_swift_visitor_mark_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorMarkCustom: HtmlVisitorProtocol {
    func visitMark(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "==\(text)==") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorMarkCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>This is a <mark>highlighted passage</mark> in the text.</p>", options: _options)

```
