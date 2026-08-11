---
id: fixture_swift_metadata_extract_all_links
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<html><head><title>Links Page</title></head><body><p>Visit <a href=\"https://example.com\">Example</a> or <a href=\"https://docs.example.com\">Docs</a>.</p><p>Also see <a href=\"/relative/path\">relative link</a> and <a href=\"mailto:hello@example.com\">email us</a>.</p></body></html>", options: _options)

```
