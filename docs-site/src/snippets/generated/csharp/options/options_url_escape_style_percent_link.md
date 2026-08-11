---
id: fixture_csharp_options_url_escape_style_percent_link
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
var result = HtmlToMarkdownConverter.Convert("<a href=\"/file (1).pdf\">file</a>", new ConversionOptions { UrlEscapeStyle = JsonSerializer.Deserialize<UrlEscapeStyle>("\"percent\"", ConfigOptions)! });

```
