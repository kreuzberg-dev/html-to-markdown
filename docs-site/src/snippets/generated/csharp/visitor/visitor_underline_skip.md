---
id: fixture_csharp_visitor_underline_skip
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using HtmlToMarkdown;

var result = HtmlToMarkdownConverter.Convert("<p>Normal text with <u>underlined part</u> and more text.</p>", new ConversionOptions());

```
