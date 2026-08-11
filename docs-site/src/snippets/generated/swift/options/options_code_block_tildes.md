---
id: fixture_swift_options_code_block_tildes
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Tildes\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code>let x = 1;</code></pre>", options: _options)

```
