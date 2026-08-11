---
id: fixture_swift_options_url_escape_style_percent_image
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"url_escape_style\":\"percent\"}")
_ = try HtmlToMarkdown.convert(html: "<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options: _options)

```
