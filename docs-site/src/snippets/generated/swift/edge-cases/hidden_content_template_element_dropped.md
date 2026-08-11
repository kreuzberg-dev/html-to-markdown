---
id: fixture_swift_hidden_content_template_element_dropped
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<p>visible</p><template><p>secret template text</p></template><p>also visible</p>", options: _options)

```
