---
id: fixture_swift_hidden_content_display_none_dropped
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", options: _options)

```
