---
id: fixture_csharp_visitor_skip_heading
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>Body text remains.</p>", new ConversionOptions());

```
