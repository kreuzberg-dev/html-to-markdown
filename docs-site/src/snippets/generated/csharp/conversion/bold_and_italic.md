---
id: fixture_csharp_bold_and_italic
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><strong><em>both</em></strong></p>", new ConversionOptions());

```
