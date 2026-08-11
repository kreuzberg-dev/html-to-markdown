---
id: fixture_swift_visitor_continue_default
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorContinueDefault: HtmlVisitorProtocol {
    func visitStrong(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .`continue` }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorContinueDefault())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Hello <strong>World</strong></p>", options: _options)

```
