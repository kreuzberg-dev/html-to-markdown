---
id: fixture_swift_options_escape_ascii_enabled
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"escape_ascii\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Text with # hash and [brackets] and * star</p>", options: _options)

```
