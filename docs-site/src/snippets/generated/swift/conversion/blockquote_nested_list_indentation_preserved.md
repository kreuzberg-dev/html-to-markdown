---
id: fixture_swift_blockquote_nested_list_indentation_preserved
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>", options: _options)

```
