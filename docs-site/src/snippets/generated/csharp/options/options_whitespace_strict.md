---
id: fixture_csharp_options_whitespace_strict
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
var result = HtmlToMarkdownConverter.Convert("<p>Preserved   spacing.</p>", new ConversionOptions { WhitespaceMode = JsonSerializer.Deserialize<WhitespaceMode>("\"Strict\"", ConfigOptions)! });

```
