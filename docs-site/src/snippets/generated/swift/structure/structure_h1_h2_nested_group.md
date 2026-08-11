---
id: fixture_swift_structure_h1_h2_nested_group
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"include_document_structure\":true}")
_ = try HtmlToMarkdown.convert(html: "<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", options: _options)

```
