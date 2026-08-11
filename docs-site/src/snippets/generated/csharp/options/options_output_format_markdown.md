---
id: fixture_csharp_options_output_format_markdown
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
var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><p>Some text.</p>", new ConversionOptions { HeadingStyle = JsonSerializer.Deserialize<HeadingStyle>("\"Atx\"", ConfigOptions)!, OutputFormat = OutputFormat.Markdown });

```
