---
id: fixture_swift_options_highlight_bold
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"highlight_style\":\"Bold\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Text with <mark>highlighted</mark> text.</p>", options: _options)

```
