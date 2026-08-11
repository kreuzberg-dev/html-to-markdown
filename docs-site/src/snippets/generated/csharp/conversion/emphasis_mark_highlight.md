---
id: fixture_csharp_emphasis_mark_highlight
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><mark>highlighted</mark></p>", new ConversionOptions());

```
