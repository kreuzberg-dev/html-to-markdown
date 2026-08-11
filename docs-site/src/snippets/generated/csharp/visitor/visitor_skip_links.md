---
id: fixture_csharp_visitor_skip_links
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Before <a href=\"https://example.com\">link text</a> after</p>", new ConversionOptions());

```
