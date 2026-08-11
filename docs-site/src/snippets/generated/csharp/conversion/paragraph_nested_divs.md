---
id: fixture_csharp_paragraph_nested_divs
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<div><div><p>Nested text</p></div></div>", new ConversionOptions());

```
