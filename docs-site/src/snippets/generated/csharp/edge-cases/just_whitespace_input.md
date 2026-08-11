---
id: fixture_csharp_just_whitespace_input
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("   ", new ConversionOptions());

```
