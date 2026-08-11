---
id: fixture_csharp_options_max_depth_zero_empty
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Hello</p>", new ConversionOptions { MaxDepth = 0 });

```
