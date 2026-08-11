---
id: fixture_swift_encoding_html_entities
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", options: _options)

```
