---
id: fixture_swift_options_sup_symbol_caret
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"sup_symbol\":\"^\"}")
_ = try HtmlToMarkdown.convert(html: "<p>x<sup>2</sup></p>", options: _options)

```
