---
id: fixture_swift_options_skip_images_true
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"skip_images\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Before <img src='test.jpg' alt='photo'> After</p>", options: _options)

```
