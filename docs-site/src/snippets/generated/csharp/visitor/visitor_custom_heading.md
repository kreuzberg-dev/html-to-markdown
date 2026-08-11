---
id: fixture_csharp_visitor_custom_heading
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h2>Section Title</h2><p>Content below heading.</p>", new ConversionOptions());

```
