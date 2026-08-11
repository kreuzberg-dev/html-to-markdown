---
id: fixture_swift_options_wrap_disabled
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"wrap\":false}")
_ = try HtmlToMarkdown.convert(html: "<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options: _options)

```
