---
id: fixture_swift_visitor_form_skip
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorFormSkip: HtmlVisitorProtocol {
    func visitForm(_ ctx: HtmlToMarkdown.NodeContext, _ action: String?, _ method: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorFormSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", options: _options)

```
