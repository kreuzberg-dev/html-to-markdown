---
id: fixture_csharp_options_escape_underscores
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>The variable_name is defined.</p>", new ConversionOptions { EscapeUnderscores = true });

```
