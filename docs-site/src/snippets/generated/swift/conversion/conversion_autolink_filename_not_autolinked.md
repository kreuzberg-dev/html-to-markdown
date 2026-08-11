---
id: fixture_swift_conversion_autolink_filename_not_autolinked
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<a href=\"foobar.png\">foobar.png</a>", options: _options)

```
