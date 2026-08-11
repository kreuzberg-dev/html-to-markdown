---
id: fixture_swift_options_escape_underscores
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"escape_underscores\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>The variable_name is defined.</p>", options: _options)

```
