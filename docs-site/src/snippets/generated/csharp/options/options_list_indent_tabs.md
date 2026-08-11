---
id: fixture_csharp_options_list_indent_tabs
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
var result = HtmlToMarkdownConverter.Convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", new ConversionOptions { ListIndentType = JsonSerializer.Deserialize<ListIndentType>("\"Tabs\"", ConfigOptions)! });

```
