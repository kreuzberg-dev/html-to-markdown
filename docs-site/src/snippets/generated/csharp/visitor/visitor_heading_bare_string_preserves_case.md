---
id: fixture_csharp_visitor_heading_bare_string_preserves_case
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h2>Important Section Title</h2><p>Body.</p>", new ConversionOptions());

```
