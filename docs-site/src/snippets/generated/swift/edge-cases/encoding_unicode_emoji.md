---
id: fixture_swift_encoding_unicode_emoji
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Hello 🌍 World 🚀</p><p>Stars: ⭐ ✨</p>", options: _options)

```
