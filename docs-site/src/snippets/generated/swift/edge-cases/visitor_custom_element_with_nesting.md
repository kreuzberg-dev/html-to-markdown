---
id: fixture_swift_visitor_custom_element_with_nesting
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorCustomElementWithNesting: HtmlVisitorProtocol {
    func visitCustomElement(_ ctx: HtmlToMarkdown.NodeContext, _ tagName: String, _ html: String) -> VisitResult { return .custom(field0: "[CUSTOM WIDGET]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorCustomElementWithNesting())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", options: _options)

```
