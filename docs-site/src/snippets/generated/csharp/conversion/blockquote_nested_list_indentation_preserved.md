---
id: fixture_csharp_blockquote_nested_list_indentation_preserved
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>", new ConversionOptions());

```
