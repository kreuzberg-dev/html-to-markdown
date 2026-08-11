---
id: fixture_csharp_visitor_line_break_skip
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", new ConversionOptions());

```
