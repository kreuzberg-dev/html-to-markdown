---
id: fixture_swift_options_heading_style_underlined
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"heading_style\":\"Underlined\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Main Title</h1>", options: _options)

```
