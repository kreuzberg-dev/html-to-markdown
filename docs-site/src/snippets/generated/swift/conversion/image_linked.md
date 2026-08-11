---
id: fixture_swift_image_linked
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", options: _options)

```
