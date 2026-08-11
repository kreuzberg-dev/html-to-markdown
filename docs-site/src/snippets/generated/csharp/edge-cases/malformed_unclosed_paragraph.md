---
id: fixture_csharp_malformed_unclosed_paragraph
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This paragraph is never closed", new ConversionOptions());

```
