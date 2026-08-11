---
id: fixture_csharp_emphasis_superscript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>x<sup>2</sup></p>", new ConversionOptions());

```
