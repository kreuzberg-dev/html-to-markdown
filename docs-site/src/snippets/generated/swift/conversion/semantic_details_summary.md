---
id: fixture_swift_semantic_details_summary
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", options: _options)

```
