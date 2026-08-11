---
id: fixture_swift_visitor_horizontal_rule_skip
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorHorizontalRuleSkip: HtmlVisitorProtocol {
    func visitHorizontalRule(_ ctx: HtmlToMarkdown.NodeContext) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorHorizontalRuleSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", options: _options)

```
