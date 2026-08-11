---
id: fixture_swift_semantic_abbr
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", options: _options)

```
