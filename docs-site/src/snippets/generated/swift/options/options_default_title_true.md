---
id: fixture_swift_options_default_title_true
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"default_title\":true}")
_ = try HtmlToMarkdown.convert(html: "<p><a href='https://example.com'>Link</a></p>", options: _options)

```
