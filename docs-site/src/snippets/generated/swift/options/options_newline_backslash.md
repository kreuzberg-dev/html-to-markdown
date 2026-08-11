---
id: fixture_swift_options_newline_backslash
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"newline_style\":\"Backslash\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Line one<br>Line two</p>", options: _options)

```
