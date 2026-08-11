---
id: fixture_swift_metadata_link_type_email_classified
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options: _options)

```
