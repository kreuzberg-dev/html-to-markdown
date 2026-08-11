---
id: fixture_csharp_visitor_line_break_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>First line<br>Second line<br>Third line</p>", new ConversionOptions());

```
