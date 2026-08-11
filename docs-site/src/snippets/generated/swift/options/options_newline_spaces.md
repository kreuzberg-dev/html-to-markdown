---
id: fixture_swift_options_newline_spaces
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"newline_style\":\"Spaces\"}")
_ = try HtmlToMarkdown.convert(html: "<p>First<br>Second</p>", options: _options)

```
