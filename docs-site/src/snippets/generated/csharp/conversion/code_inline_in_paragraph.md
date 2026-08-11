---
id: fixture_csharp_code_inline_in_paragraph
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Call the <code>initialize()</code> method first.</p>", new ConversionOptions());

```
