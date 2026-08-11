---
id: fixture_csharp_visitor_custom_link_static
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"https://example.com\">Click here</a>", new ConversionOptions());

```
