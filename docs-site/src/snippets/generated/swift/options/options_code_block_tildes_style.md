---
id: fixture_swift_options_code_block_tildes_style
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Tildes\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code>some code</code></pre>", options: _options)

```
