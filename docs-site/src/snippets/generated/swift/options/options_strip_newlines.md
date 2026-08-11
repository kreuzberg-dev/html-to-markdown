---
id: fixture_swift_options_strip_newlines
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"strip_newlines\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>First paragraph.</p><p>Second paragraph.</p>", options: _options)

```
