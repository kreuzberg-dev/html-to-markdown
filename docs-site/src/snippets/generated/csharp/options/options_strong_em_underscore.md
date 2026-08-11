---
id: fixture_csharp_options_strong_em_underscore
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><strong>bold</strong> and <em>italic</em></p>", new ConversionOptions { StrongEmSymbol = "_" });

```
