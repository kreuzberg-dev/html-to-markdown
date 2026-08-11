---
id: fixture_csharp_options_sub_symbol_tilde
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>H<sub>2</sub>O</p>", new ConversionOptions { SubSymbol = "~" });

```
