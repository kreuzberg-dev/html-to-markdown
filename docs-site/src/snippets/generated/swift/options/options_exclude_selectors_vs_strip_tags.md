---
id: fixture_swift_options_exclude_selectors_vs_strip_tags
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".wrapper\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options: _options)

```
