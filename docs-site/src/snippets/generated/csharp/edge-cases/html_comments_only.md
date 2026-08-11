---
id: fixture_csharp_html_comments_only
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<!-- This is a comment --><!-- Another comment -->", new ConversionOptions());

```
