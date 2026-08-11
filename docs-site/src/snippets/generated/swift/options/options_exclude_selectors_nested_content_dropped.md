---
id: fixture_swift_options_exclude_selectors_nested_content_dropped
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".sidebar\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><aside class=\"sidebar\"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>", options: _options)

```
