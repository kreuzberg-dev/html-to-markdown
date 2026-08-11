---
id: fixture_csharp_inline_code
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Use <code>console.log()</code> to debug</p>", new ConversionOptions());

```
