---
id: fixture_swift_options_escape_asterisks
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"escape_asterisks\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Use 2*3 = 6 in math.</p>", options: _options)

```
