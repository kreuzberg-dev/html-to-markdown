---
id: fixture_csharp_table_nested_chain_not_misclassified_as_layout
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<table><tr><td><table><tr><td><table><tr><td>leaf</td></tr></table></td></tr></table></td></tr></table>", new ConversionOptions());

```
