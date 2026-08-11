---
id: fixture_swift_options_convert_as_inline
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"convert_as_inline\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>One</p><p>Two</p>", options: _options)

```
