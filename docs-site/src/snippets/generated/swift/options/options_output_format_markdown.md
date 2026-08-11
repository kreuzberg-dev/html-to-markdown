---
id: fixture_swift_options_output_format_markdown
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"heading_style\":\"Atx\",\"output_format\":\"Markdown\"}")
_ = try HtmlToMarkdown.convert(html: "<h1>Title</h1><p>Some text.</p>", options: _options)

```
