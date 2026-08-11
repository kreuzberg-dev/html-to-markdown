---
id: fixture_csharp_visitor_custom_blockquote
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><p>A wise quote.</p></blockquote>", new ConversionOptions());

```
