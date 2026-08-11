---
id: fixture_swift_options_preprocessing_remove_forms
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"remove_forms\":true}}")
_ = try HtmlToMarkdown.convert(html: "<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options: _options)

```
