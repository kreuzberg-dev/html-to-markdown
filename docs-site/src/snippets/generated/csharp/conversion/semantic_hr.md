---
id: fixture_csharp_semantic_hr
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Above</p><hr><p>Below</p>", new ConversionOptions());

```
