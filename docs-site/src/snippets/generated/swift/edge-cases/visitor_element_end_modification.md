---
id: fixture_swift_visitor_element_end_modification
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorElementEndModification: HtmlVisitorProtocol {
    func visitElementEnd(_ ctx: HtmlToMarkdown.NodeContext, _ output: String) -> VisitResult { return .custom(field0: "MODIFIED OUTPUT") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorElementEndModification())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<blockquote><p>Original quote</p></blockquote>", options: _options)

```
