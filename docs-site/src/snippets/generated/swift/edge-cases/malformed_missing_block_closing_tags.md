---
id: fixture_swift_malformed_missing_block_closing_tags
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<div><h1>Title<p>First paragraph<p>Second paragraph</div>", options: _options)

```
