---
id: fixture_csharp_visitor_underline_custom
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This is <u>very important</u> text.</p>", new ConversionOptions());

```
