---
id: fixture_csharp_blockquote_simple
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><p>Quote text</p></blockquote>", new ConversionOptions());

```
