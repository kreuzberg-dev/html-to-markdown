---
id: fixture_swift_options_heading_style_atx
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"heading_style\":\"Atx\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><h2>Subtitle</h2>", options: _options)

```
