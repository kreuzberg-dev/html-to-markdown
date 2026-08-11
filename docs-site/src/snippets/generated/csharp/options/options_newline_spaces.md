---
id: fixture_csharp_options_newline_spaces
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
var result = HtmlToMarkdownConverter.Convert("<p>First<br>Second</p>", new ConversionOptions { NewlineStyle = JsonSerializer.Deserialize<NewlineStyle>("\"Spaces\"", ConfigOptions)! });

```
