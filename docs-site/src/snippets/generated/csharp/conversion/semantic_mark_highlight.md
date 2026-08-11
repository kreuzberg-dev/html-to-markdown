---
id: fixture_csharp_semantic_mark_highlight
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>This is <mark>highlighted text</mark> in a sentence.</p>", new ConversionOptions());

```
