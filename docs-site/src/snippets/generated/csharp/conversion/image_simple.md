---
id: fixture_csharp_image_simple
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<img src=\"photo.jpg\" alt=\"A photo\">", new ConversionOptions());

```
