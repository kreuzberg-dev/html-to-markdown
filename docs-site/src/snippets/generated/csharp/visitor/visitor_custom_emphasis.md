---
id: fixture_csharp_visitor_custom_emphasis
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This is <em>important</em> text.</p>", new ConversionOptions());

```
