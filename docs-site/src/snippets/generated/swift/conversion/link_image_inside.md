---
id: fixture_swift_link_image_inside
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", options: _options)

```
