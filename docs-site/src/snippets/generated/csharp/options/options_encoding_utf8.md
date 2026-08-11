---
id: fixture_csharp_options_encoding_utf8
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Café naïve résumé</p>", new ConversionOptions { Encoding = "utf-8" });

```
