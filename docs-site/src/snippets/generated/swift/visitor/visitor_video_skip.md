---
id: fixture_swift_visitor_video_skip
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorVideoSkip: HtmlVisitorProtocol {
    func visitVideo(_ ctx: HtmlToMarkdown.NodeContext, _ src: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorVideoSkip())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", options: _options)

```
