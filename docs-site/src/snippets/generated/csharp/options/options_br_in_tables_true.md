---
id: fixture_csharp_options_br_in_tables_true
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", new ConversionOptions { BrInTables = true });

```
