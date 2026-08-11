---
id: fixture_csharp_line_break_br_tag
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>First line.<br>Second line.</p>", new ConversionOptions());

```
