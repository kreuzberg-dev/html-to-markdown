---
id: fixture_csharp_semantic_abbr
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", new ConversionOptions());

```
