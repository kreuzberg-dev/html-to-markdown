---
id: fixture_swift_options_heading_style_atx_closed
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"heading_style\":\"AtxClosed\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Closed Heading</h1>", options: _options)

```
