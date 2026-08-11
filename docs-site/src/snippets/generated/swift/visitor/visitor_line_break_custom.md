---
id: fixture_swift_visitor_line_break_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorLineBreakCustom: HtmlVisitorProtocol {
    func visitLineBreak(_ ctx: HtmlToMarkdown.NodeContext) -> VisitResult { return .custom(field0: " | ") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorLineBreakCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>First line<br>Second line<br>Third line</p>", options: _options)

```
