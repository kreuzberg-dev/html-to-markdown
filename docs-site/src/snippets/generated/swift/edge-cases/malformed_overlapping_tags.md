---
id: fixture_swift_malformed_overlapping_tags
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p><b><i>bold and italic</b></i></p>", options: _options)

```
