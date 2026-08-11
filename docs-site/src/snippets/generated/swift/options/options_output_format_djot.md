---
id: fixture_swift_options_output_format_djot
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"output_format\":\"Djot\"}")
_ = try HtmlToMarkdown.convert(html: "<p>Simple paragraph.</p>", options: _options)

```
