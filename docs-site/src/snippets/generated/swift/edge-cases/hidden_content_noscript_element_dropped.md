---
id: fixture_swift_hidden_content_noscript_element_dropped
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", options: _options)

```
