---
id: fixture_swift_options_include_document_structure_false
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"include_document_structure\":false}")
_ = try HtmlToMarkdown.convert(html: "<article><h1>Heading</h1><p>Paragraph body.</p></article>", options: _options)

```
