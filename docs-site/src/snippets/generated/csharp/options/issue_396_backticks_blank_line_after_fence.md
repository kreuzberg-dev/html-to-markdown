---
id: fixture_csharp_issue_396_backticks_blank_line_after_fence
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
var result = HtmlToMarkdownConverter.Convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", new ConversionOptions { CodeBlockStyle = JsonSerializer.Deserialize<CodeBlockStyle>("\"Backticks\"", ConfigOptions)! });

```
