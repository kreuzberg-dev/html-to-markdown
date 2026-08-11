---
id: fixture_csharp_line_break_multiple_br
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Start.<br><br>End.</p>", new ConversionOptions());

```
