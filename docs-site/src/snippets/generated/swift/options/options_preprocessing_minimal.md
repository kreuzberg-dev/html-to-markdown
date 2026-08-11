---
id: fixture_swift_options_preprocessing_minimal
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"preset\":\"Minimal\"}}")
_ = try HtmlToMarkdown.convert(html: "<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", options: _options)

```
