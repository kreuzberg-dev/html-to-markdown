---
id: fixture_swift_visitor_skip_images
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

final class LocalVisitor_VisitorSkipImages: HtmlVisitorProtocol {
    func visitImage(_ ctx: HtmlToMarkdown.NodeContext, _ src: String, _ alt: String, _ title: String?) -> VisitResult { return .skip }
}

let _visitorHandle_options = makeHtmlVisitorHandle(LocalVisitor_VisitorSkipImages())
let _options = try HtmlToMarkdown.conversionOptionsFromJsonWithVisitor("{}", _visitorHandle_options)
_ = try HtmlToMarkdown.convert(html: "<p>Before image</p><img src=\"photo.jpg\" alt=\"A photo\"><p>After image</p>", options: _options)

```
