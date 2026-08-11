---
id: fixture_csharp_ordered_list
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", new ConversionOptions());

```
