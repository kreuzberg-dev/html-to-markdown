---
id: fixture_swift_structure_list
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"include_document_structure\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", options: _options)

```
