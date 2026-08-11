---
id: fixture_csharp_options_max_depth_truncates
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", new ConversionOptions { MaxDepth = 3 });

```
