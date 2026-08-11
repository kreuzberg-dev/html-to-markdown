---
id: fixture_swift_blockquote_code_block_indentation_preserved
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", options: _options)

```
