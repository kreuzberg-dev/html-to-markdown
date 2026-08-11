---
id: fixture_swift_encoding_numeric_entities
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>", options: _options)

```
