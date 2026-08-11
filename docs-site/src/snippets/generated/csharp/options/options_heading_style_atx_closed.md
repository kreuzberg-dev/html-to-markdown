---
id: fixture_csharp_options_heading_style_atx_closed
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
var result = HtmlToMarkdownConverter.Convert("<h1>Closed Heading</h1>", new ConversionOptions { HeadingStyle = JsonSerializer.Deserialize<HeadingStyle>("\"AtxClosed\"", ConfigOptions)! });

```
