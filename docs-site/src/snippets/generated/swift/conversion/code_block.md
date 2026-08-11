---
id: fixture_swift_code_block
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<pre><code class=\"language-python\">print('hello')</code></pre>", options: _options)

```
