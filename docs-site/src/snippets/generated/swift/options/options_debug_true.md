---
id: fixture_swift_options_debug_true
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"debug\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Debug test</p>", options: _options)

```
