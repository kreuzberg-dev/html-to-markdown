---
id: fixture_csharp_options_heading_style_atx
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
var result = HtmlToMarkdownConverter.Convert("<h1>Title</h1><h2>Subtitle</h2>", new ConversionOptions { HeadingStyle = JsonSerializer.Deserialize<HeadingStyle>("\"Atx\"", ConfigOptions)! });

```
