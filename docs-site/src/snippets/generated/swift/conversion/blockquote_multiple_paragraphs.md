---
id: fixture_swift_blockquote_multiple_paragraphs
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", options: _options)

```
