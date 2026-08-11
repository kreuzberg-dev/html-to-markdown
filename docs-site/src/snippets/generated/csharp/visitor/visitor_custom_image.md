---
id: fixture_csharp_visitor_custom_image
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<img src=\"banner.png\" alt=\"Banner\">", new ConversionOptions());

```
