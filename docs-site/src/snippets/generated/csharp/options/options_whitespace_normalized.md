---
id: fixture_csharp_options_whitespace_normalized
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
var result = HtmlToMarkdownConverter.Convert("<p>Text   with    extra   spaces.</p>", new ConversionOptions { WhitespaceMode = JsonSerializer.Deserialize<WhitespaceMode>("\"Normalized\"", ConfigOptions)! });

```
