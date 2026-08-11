---
id: fixture_csharp_line_break_hr_tag
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before rule.</p><hr><p>After rule.</p>", new ConversionOptions());

```
