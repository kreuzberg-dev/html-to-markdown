---
id: fixture_swift_options_exclude_selectors_multiple
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"exclude_selectors\":[\".nav\",\"footer\"]}")
_ = try HtmlToMarkdown.convert(html: "<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options: _options)

```
