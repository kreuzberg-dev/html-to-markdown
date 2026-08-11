---
id: fixture_swift_options_max_depth_truncates
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"max_depth\":3}")
_ = try HtmlToMarkdown.convert(html: "<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options: _options)

```
