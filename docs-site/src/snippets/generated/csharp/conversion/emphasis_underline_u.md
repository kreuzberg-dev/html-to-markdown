---
id: fixture_csharp_emphasis_underline_u
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><u>underlined</u></p>", new ConversionOptions());

```
