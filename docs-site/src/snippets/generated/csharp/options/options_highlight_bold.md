---
id: fixture_csharp_options_highlight_bold
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using System.Text.Json;
using HtmlToMarkdown;

var ConfigOptions = new JsonSerializerOptions { PropertyNameCaseInsensitive = true };
var result = HtmlToMarkdownConverter.Convert("<p>Text with <mark>highlighted</mark> text.</p>", new ConversionOptions { HighlightStyle = JsonSerializer.Deserialize<HighlightStyle>("\"Bold\"", ConfigOptions)! });

```
