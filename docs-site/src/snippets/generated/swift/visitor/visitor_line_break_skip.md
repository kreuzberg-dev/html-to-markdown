---
id: fixture_swift_visitor_line_break_skip
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorLineBreakSkip: HtmlVisitorProtocol {
    func visitLineBreak(_ ctx: HtmlToMarkdown.NodeContext) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorLineBreakSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", options: _options)

```
