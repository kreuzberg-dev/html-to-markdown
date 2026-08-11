---
id: fixture_swift_options_list_custom_bullets
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"bullets\":\"*\"}")
_ = try HtmlToMarkdown.convert(html: "<ul><li>Item A</li><li>Item B</li></ul>", options: _options)

```
