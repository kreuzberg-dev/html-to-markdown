---
id: fixture_swift_options_whitespace_normalized
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"whitespace_mode\":\"Normalized\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Text   with    extra   spaces.</p>", options: _options)

```
