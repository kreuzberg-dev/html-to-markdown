---
id: fixture_csharp_visitor_mark_skip
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", new ConversionOptions());

```
