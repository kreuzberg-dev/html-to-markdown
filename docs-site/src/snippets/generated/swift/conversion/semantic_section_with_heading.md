---
id: fixture_swift_semantic_section_with_heading
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<section><h3>Section Heading</h3><p>Section content.</p></section>", options: _options)

```
