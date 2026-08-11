---
id: fixture_csharp_options_preprocessing_minimal
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
var result = HtmlToMarkdownConverter.Convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", new ConversionOptions { Preprocessing = new PreprocessingOptions { Preset = JsonSerializer.Deserialize<PreprocessingPreset>("\"Minimal\"", ConfigOptions)! } });

```
