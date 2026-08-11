---
id: fixture_csharp_options_escape_misc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Use # and | and ~ in text.</p>", new ConversionOptions { EscapeMisc = true });

```
