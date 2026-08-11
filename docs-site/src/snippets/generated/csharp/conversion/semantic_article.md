---
id: fixture_csharp_semantic_article
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<article><h2>Article Title</h2><p>Article body.</p></article>", new ConversionOptions());

```
