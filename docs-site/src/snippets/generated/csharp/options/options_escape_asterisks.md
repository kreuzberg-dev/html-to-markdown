---
id: fixture_csharp_options_escape_asterisks
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Use 2*3 = 6 in math.</p>", new ConversionOptions { EscapeAsterisks = true });

```
