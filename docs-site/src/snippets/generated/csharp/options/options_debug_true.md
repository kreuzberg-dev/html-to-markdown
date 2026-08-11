---
id: fixture_csharp_options_debug_true
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Debug test</p>", new ConversionOptions { Debug = true });

```
