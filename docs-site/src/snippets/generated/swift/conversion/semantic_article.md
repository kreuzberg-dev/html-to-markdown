---
id: fixture_swift_semantic_article
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<article><h2>Article Title</h2><p>Article body.</p></article>", options: _options)

```
