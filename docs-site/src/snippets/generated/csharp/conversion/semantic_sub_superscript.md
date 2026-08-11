---
id: fixture_csharp_semantic_sub_superscript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", new ConversionOptions());

```
