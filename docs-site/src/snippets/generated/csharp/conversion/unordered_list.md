---
id: fixture_csharp_unordered_list
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", new ConversionOptions());

```
