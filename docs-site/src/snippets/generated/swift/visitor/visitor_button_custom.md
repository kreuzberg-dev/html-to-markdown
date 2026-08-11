---
id: fixture_swift_visitor_button_custom
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorButtonCustom: HtmlVisitorProtocol {
    func visitButton(_ ctx: HtmlToMarkdown.NodeContext, _ text: String) -> VisitResult { return .custom(field0: "[BTN:\(text)]") }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorButtonCustom())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Confirm action: <button type=\"submit\">Click me</button> or <button type=\"reset\">Cancel</button></p>", options: _options)

```
