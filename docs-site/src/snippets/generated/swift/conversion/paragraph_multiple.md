---
id: fixture_swift_paragraph_multiple
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>First paragraph.</p><p>Second paragraph.</p>", options: _options)

```
