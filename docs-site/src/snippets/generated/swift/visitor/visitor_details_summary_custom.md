---
id: fixture_swift_visitor_details_summary_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorDetailsSummaryCustom: HtmlVisitorProtocol {
    func visitSummary(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "[EXPANDABLE] \(text)") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorDetailsSummaryCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<details><summary>Click to expand</summary><p>This content is initially hidden.</p><p>But can be revealed by the user.</p></details>", options: _options)

```
