---
id: fixture_swift_visitor_link_bare_string_preserves_case
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorLinkBareStringPreservesCase: HtmlVisitorProtocol {
    func visitLink(_ ctx: HtmlToMarkdown.NodeContext, _ href: String, _ text: String, _ title: String?) -> VisitResult { return .custom(field0: "[\(text)](https://new-cdn.com/file.pdf)") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorLinkBareStringPreservesCase())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<a href=\"https://old-cdn.com/file.pdf\">Download</a>", options: _options)

```
