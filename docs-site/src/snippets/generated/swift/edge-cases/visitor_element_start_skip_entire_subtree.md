---
id: fixture_swift_visitor_element_start_skip_entire_subtree
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorElementStartSkipEntireSubtree: HtmlVisitorProtocol {
    func visitElementStart(_ ctx: HtmlToMarkdown.NodeContext) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorElementStartSkipEntireSubtree())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<div><h1>Title</h1><p>Content</p></div>", options: _options)

```
