---
id: fixture_csharp_options_list_custom_bullets
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li>Item A</li><li>Item B</li></ul>", new ConversionOptions { Bullets = "*" });

```
