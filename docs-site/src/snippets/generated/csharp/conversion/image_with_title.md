---
id: fixture_csharp_image_with_title
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", new ConversionOptions());

```
