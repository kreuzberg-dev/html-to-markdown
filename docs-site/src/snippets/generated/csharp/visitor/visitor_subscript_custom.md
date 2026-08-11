---
id: fixture_csharp_visitor_subscript_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>H<sub>2</sub>O is water.</p>", new ConversionOptions());

```
