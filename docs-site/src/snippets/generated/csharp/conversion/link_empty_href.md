---
id: fixture_csharp_link_empty_href
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"\">No destination</a>", new ConversionOptions());

```
