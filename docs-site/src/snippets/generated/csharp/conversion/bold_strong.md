---
id: fixture_csharp_bold_strong
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><strong>bold</strong></p>", new ConversionOptions());

```
