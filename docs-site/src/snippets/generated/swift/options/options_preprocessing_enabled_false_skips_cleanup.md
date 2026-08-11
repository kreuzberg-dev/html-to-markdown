---
id: fixture_swift_options_preprocessing_enabled_false_skips_cleanup
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"enabled\":false}}")
_ = try HtmlToMarkdown.convert(html: "<nav>NavSection</nav><p>Paragraph</p>", options: _options)

```
