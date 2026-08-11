---
id: fixture_swift_result_tables_without_structure_flag
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", options: _options)

```
