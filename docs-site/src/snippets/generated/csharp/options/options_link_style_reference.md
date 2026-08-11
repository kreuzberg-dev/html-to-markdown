---
id: fixture_csharp_options_link_style_reference
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
var result = HtmlToMarkdownConverter.Convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", new ConversionOptions { LinkStyle = JsonSerializer.Deserialize<LinkStyle>("\"Reference\"", ConfigOptions)! });

```
