---
id: fixture_swift_semantic_mark_highlight
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>This is <mark>highlighted text</mark> in a sentence.</p>", options: _options)

```
