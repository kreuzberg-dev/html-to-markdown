---
id: fixture_csharp_visitor_continue_default
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello <strong>World</strong></p>", new ConversionOptions());

```
