---
id: fixture_csharp_semantic_details_summary
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", new ConversionOptions());

```
