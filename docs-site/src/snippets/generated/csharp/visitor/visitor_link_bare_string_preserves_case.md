---
id: fixture_csharp_visitor_link_bare_string_preserves_case
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", new ConversionOptions());

```
