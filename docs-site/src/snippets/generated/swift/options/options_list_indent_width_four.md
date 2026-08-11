---
id: fixture_swift_options_list_indent_width_four
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"list_indent_width\":4}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", options: _options)

```
