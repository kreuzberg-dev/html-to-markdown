---
id: fixture_swift_image_simple
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<img src=\"photo.jpg\" alt=\"A photo\">", options: _options)

```
