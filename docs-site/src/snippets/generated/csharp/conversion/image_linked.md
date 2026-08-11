---
id: fixture_csharp_image_linked
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", new ConversionOptions());

```
