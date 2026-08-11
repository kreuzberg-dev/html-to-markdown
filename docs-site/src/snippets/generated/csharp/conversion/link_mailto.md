---
id: fixture_csharp_link_mailto
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"mailto:user@example.com\">Email us</a>", new ConversionOptions());

```
