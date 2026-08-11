---
id: fixture_csharp_visitor_image_bare_string_preserves_case
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", new ConversionOptions());

```
