---
id: fixture_csharp_visitor_element_end_modification
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<blockquote><p>Original quote</p></blockquote>", new ConversionOptions());

```
