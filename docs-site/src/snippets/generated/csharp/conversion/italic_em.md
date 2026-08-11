---
id: fixture_csharp_italic_em
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><em>italic</em></p>", new ConversionOptions());

```
