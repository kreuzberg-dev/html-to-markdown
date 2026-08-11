---
id: fixture_csharp_image_no_alt
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<img src=\"banner.jpg\">", new ConversionOptions());

```
