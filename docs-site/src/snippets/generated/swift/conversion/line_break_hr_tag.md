---
id: fixture_swift_line_break_hr_tag
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Before rule.</p><hr><p>After rule.</p>", options: _options)

```
