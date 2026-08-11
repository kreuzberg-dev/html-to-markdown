---
id: fixture_swift_script_tags_only
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", options: _options)

```
