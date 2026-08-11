---
id: fixture_swift_blockquote_text_then_paragraph_gets_blank_line
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<blockquote>Just text, then <p>a paragraph</p></blockquote>", options: _options)

```
