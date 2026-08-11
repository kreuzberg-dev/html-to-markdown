---
id: fixture_csharp_conversion_autolink_https_url
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"https://example.com\">https://example.com</a>", new ConversionOptions());

```
