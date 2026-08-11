---
id: fixture_csharp_metadata_microdata_schema_organization
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head><title>Company</title></head><body><div itemscope itemtype=\"https://schema.org/Organization\"><span itemprop=\"name\">Acme Corp</span><span itemprop=\"foundingDate\">2020</span><span itemprop=\"url\">https://acmecorp.example.com</span><span itemprop=\"logo\">https://acmecorp.example.com/logo.png</span></div></body></html>", new ConversionOptions { ExtractMetadata = true });

```
