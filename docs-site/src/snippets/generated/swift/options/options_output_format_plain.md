---
id: fixture_swift_options_output_format_plain
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"output_format\":\"Plain\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options: _options)

```
