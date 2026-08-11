---
id: fixture_csharp_visitor_horizontal_rule_skip
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", new ConversionOptions());

```
