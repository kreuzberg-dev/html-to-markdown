---
id: fixture_swift_twitter_card_tags
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{}")
_ = try HtmlToMarkdown.convert(html: "<html><head><meta name=\"twitter:card\" content=\"summary_large_image\"><meta name=\"twitter:site\" content=\"@examplesite\"><meta name=\"twitter:title\" content=\"Twitter Card Title\"><meta name=\"twitter:description\" content=\"Twitter card description.\"><meta name=\"twitter:image\" content=\"https://example.com/twitter-image.jpg\"></head><body><p>Content</p></body></html>", options: _options)

```
