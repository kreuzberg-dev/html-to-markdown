---
id: fixture_swift_options_exclude_selectors_empty_noop
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[]}")
_ = try HtmlToMarkdown.convert(html: "<p>Hello world</p>", options: _options)

```
