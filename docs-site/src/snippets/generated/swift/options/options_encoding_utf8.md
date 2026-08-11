---
id: fixture_swift_options_encoding_utf8
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"encoding\":\"utf-8\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Café naïve résumé</p>", options: _options)

```
