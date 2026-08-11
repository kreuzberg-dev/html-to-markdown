---
id: fixture_csharp_options_exclude_selectors_vs_strip_tags
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", new ConversionOptions { ExcludeSelectors = new List<string> { ".wrapper" } });

```
