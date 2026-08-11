---
id: fixture_csharp_smoke_simple_paragraph
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello World</p>", new ConversionOptions());

```
