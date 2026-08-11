---
id: fixture_csharp_options_list_indent_width_four
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", new ConversionOptions { ListIndentWidth = 4 });

```
