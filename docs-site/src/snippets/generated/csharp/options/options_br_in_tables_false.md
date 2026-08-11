---
id: fixture_csharp_options_br_in_tables_false
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", new ConversionOptions { BrInTables = false });

```
