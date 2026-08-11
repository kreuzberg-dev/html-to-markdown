---
id: fixture_swift_hidden_content_visibility_hidden_dropped
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", options: _options)

```
