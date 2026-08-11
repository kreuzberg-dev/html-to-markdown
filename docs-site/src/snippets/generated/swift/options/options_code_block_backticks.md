---
id: fixture_swift_options_code_block_backticks
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"code_block_style\":\"Backticks\"}")
_ = try HtmlToMarkdown.convert(html: "<pre><code class=\"language-js\">console.log('hi');</code></pre>", options: _options)

```
