---
id: fixture_swift_form_textarea
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"preprocessing\":{\"remove_forms\":false}}")
_ = try HtmlToMarkdown.convert(html: "<form><label>Message:</label><textarea>Default text content</textarea></form>", options: _options)

```
