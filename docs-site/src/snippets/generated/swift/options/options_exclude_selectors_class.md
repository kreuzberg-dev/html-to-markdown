---
id: fixture_swift_options_exclude_selectors_class
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".cookie-banner\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options: _options)

```
