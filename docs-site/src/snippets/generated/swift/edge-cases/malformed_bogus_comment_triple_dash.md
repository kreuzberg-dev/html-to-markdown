---
id: fixture_swift_malformed_bogus_comment_triple_dash
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", options: _options)

```
