---
id: fixture_csharp_paragraph_multiple
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>First paragraph.</p><p>Second paragraph.</p>", new ConversionOptions());

```
