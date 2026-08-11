---
id: fixture_swift_options_list_indent_tabs
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"list_indent_type\":\"Tabs\"}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options: _options)

```
