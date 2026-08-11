---
id: fixture_swift_xss_script_tag_stripped
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", options: _options)

```
