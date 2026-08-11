---
id: fixture_csharp_visitor_mark_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This is a <mark>highlighted passage</mark> in the text.</p>", new ConversionOptions());

```
