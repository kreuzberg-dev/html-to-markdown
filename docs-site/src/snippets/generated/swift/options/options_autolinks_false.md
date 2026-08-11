---
id: fixture_swift_options_autolinks_false
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"autolinks\":false}")
_ = try HtmlToMarkdown.convert(html: "<p><a href='https://example.com'>https://example.com</a></p>", options: _options)

```
