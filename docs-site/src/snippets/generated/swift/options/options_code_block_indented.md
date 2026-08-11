---
id: fixture_swift_options_code_block_indented
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Indented\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code>print('hello')</code></pre>", options: _options)

```
