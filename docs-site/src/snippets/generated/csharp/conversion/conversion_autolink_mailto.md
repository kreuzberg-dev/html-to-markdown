---
id: fixture_csharp_conversion_autolink_mailto
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"mailto:a@b.com\">a@b.com</a>", new ConversionOptions());

```
