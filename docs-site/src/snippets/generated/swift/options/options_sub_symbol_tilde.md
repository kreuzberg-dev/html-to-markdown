---
id: fixture_swift_options_sub_symbol_tilde
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"sub_symbol\":\"~\"}")
_ = try HtmlToMarkdown.convert(html: "<p>H<sub>2</sub>O</p>", options: _options)

```
