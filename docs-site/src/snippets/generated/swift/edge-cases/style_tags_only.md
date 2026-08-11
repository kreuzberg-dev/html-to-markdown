---
id: fixture_swift_style_tags_only
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>", options: _options)

```
