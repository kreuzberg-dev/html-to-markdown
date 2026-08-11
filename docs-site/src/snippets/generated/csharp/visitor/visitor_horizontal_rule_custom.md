---
id: fixture_csharp_visitor_horizontal_rule_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", new ConversionOptions());

```
