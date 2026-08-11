---
id: fixture_swift_metadata_microdata_schema_organization
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import HtmlToMarkdown

let _options = try HtmlToMarkdown.conversionOptionsFromJson("{\"extract_metadata\":true}")
_ = try HtmlToMarkdown.convert(html: "<html><head><title>Company</title></head><body><div itemscope itemtype=\"https://schema.org/Organization\"><span itemprop=\"name\">Acme Corp</span><span itemprop=\"foundingDate\">2020</span><span itemprop=\"url\">https://acmecorp.example.com</span><span itemprop=\"logo\">https://acmecorp.example.com/logo.png</span></div></body></html>", options: _options)

```
