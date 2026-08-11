---
id: fixture_swift_options_br_in_tables_true
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"br_in_tables\":true}")
_ = try HtmlToMarkdown.convert(html: "<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options: _options)

```
