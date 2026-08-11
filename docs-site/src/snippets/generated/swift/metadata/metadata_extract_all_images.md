---
id: fixture_swift_metadata_extract_all_images
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<html><head><title>Gallery</title></head><body><img src=\"https://example.com/photo1.jpg\" alt=\"Photo 1\"><img src=\"https://example.com/photo2.png\" alt=\"Photo 2\"><img src=\"/local/image.webp\" alt=\"Local image\"></body></html>", options: _options)

```
