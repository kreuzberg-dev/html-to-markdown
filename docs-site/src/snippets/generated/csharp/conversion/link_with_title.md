---
id: fixture_csharp_link_with_title
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<a href=\"https://example.com\" title=\"Example Site\">Example</a>", new ConversionOptions());

```
