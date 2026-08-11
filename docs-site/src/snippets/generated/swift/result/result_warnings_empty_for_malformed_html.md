---
id: fixture_swift_result_warnings_empty_for_malformed_html
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Unclosed paragraph<div>Mixed nesting</p></div>", options: _options)

```
