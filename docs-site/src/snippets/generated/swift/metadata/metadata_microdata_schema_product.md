---
id: fixture_swift_metadata_microdata_schema_product
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<html><head><title>Product</title></head><body><div itemscope itemtype=\"https://schema.org/Product\"><h1 itemprop=\"name\">Awesome Widget</h1><span itemprop=\"description\">The best widget on the market</span><span itemprop=\"price\">29.99</span><span itemprop=\"priceCurrency\">USD</span><img itemprop=\"image\" src=\"widget.jpg\" alt=\"Widget\"><span itemprop=\"ratingValue\">4.5</span></div></body></html>", options: _options)

```
