---
id: fixture_csharp_empty_html
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<html><head></head><body></body></html>", new ConversionOptions());

```
