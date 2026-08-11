---
id: fixture_swift_options_exclude_selectors_id
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\"#ad-container\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options: _options)

```
