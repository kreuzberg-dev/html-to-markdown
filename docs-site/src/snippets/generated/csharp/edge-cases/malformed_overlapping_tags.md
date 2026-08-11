---
id: fixture_csharp_malformed_overlapping_tags
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><b><i>bold and italic</b></i></p>", new ConversionOptions());

```
