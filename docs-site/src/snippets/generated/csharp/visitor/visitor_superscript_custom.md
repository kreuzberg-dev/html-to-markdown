---
id: fixture_csharp_visitor_superscript_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Einstein's E=mc<sup>2</sup> revolutionized physics.</p>", new ConversionOptions());

```
