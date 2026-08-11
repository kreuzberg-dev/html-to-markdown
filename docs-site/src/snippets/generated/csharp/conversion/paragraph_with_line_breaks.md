---
id: fixture_csharp_paragraph_with_line_breaks
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Line one.<br>Line two.<br>Line three.</p>", new ConversionOptions());

```
