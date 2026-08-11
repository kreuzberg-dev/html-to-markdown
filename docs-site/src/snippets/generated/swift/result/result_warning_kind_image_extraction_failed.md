---
id: fixture_swift_result_warning_kind_image_extraction_failed
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_images\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options: _options)

```
