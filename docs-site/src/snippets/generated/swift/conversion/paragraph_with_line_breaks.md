---
id: fixture_swift_paragraph_with_line_breaks
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Line one.<br>Line two.<br>Line three.</p>", options: _options)

```
