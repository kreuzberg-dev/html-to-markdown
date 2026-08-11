---
id: fixture_swift_options_max_depth_zero_empty
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"max_depth\":0}")
_ = try HtmlToMarkdown.convert(html: "<p>Hello</p>", options: _options)

```
