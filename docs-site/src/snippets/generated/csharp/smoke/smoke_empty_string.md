---
id: fixture_csharp_smoke_empty_string
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("", new ConversionOptions());

```
