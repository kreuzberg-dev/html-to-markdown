---
id: fixture_swift_options_br_in_tables_false
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"br_in_tables\":false}")
_ = try HtmlToMarkdown.convert(html: "<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options: _options)

```
