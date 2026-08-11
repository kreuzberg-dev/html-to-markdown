---
id: fixture_csharp_options_autolinks_false
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p><a href='https://example.com'>https://example.com</a></p>", new ConversionOptions { Autolinks = false });

```
