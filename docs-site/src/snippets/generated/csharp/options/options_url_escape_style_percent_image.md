---
id: fixture_csharp_options_url_escape_style_percent_image
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
var result = HtmlToMarkdownConverter.Convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", new ConversionOptions { UrlEscapeStyle = JsonSerializer.Deserialize<UrlEscapeStyle>("\"percent\"", ConfigOptions)! });

```
