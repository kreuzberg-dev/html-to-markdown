---
id: fixture_csharp_options_output_format_plain
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", new ConversionOptions { OutputFormat = OutputFormat.Plain });

```
