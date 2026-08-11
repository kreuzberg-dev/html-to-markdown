---
id: fixture_csharp_visitor_skip_strong
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Normal <strong>bold text</strong> normal</p>", new ConversionOptions());

```
