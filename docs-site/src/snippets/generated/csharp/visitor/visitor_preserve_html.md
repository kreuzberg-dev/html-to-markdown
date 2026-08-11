---
id: fixture_csharp_visitor_preserve_html
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<div><custom-tag>Custom content</custom-tag></div>", new ConversionOptions());

```
