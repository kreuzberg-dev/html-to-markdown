---
id: fixture_swift_options_exclude_selectors_plain_text_mode
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".nav\"],\"output_format\":\"Plain\"}")
_ = try HtmlToMarkdown.convert(html: "<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options: _options)

```
