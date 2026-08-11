---
id: fixture_swift_visitor_custom_blockquote
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomBlockquote: HtmlVisitorProtocol {
    func visitBlockquote(_ ctx: HtmlToMarkdown.NodeContext, _ content: String, _ depth: UInt) -> VisitResult { return .custom(field0: "QUOTE: \"\(content)\"") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomBlockquote())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<blockquote><p>A wise quote.</p></blockquote>", options: _options)

```
