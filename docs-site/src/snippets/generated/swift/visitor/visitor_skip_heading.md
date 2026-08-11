---
id: fixture_swift_visitor_skip_heading
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSkipHeading: HtmlVisitorProtocol {
    func visitHeading(_ ctx: HtmlToMarkdown.NodeContext, _ level: UInt32, _ text: String, _ id: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSkipHeading())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><p>Body text remains.</p>", options: _options)

```
