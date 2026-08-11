---
id: fixture_swift_link_with_bold_text
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"https://example.com\"><strong>Bold link</strong></a>", options: _options)

```
