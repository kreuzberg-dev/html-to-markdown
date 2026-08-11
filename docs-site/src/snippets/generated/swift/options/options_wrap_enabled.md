---
id: fixture_swift_options_wrap_enabled
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"wrap\":true,\"wrap_width\":40}")
_ = try HtmlToMarkdown.convert(html: "<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options: _options)

```
