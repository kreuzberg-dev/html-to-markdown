---
id: fixture_swift_code_with_backticks_in_content
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Use <code>`backtick` here</code> carefully.</p>", options: _options)

```
