---
id: fixture_csharp_whitespace_only
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>   </p>", new ConversionOptions());

```
