---
id: fixture_swift_options_exclude_selectors_attribute
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\"[role='complementary']\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options: _options)

```
