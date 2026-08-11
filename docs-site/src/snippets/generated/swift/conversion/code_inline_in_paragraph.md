---
id: fixture_swift_code_inline_in_paragraph
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Call the <code>initialize()</code> method first.</p>", options: _options)

```
