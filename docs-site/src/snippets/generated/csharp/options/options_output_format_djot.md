---
id: fixture_csharp_options_output_format_djot
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Simple paragraph.</p>", new ConversionOptions { OutputFormat = OutputFormat.Djot });

```
