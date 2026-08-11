---
id: fixture_swift_options_link_style_reference
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"link_style\":\"Reference\"}")
_ = try HtmlToMarkdown.convert(html: "<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options: _options)

```
