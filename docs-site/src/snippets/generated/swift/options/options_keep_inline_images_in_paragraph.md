---
id: fixture_swift_options_keep_inline_images_in_paragraph
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"keep_inline_images_in\":[\"p\"]}")
_ = try HtmlToMarkdown.convert(html: "<p>Text <img src='icon.png' alt='icon'> more text</p>", options: _options)

```
