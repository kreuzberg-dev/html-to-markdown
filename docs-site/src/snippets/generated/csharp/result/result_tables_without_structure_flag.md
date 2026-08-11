---
id: fixture_csharp_result_tables_without_structure_flag
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", new ConversionOptions());

```
